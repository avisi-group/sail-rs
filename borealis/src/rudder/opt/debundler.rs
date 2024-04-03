use std::rc::Rc;

use {common::HashMap, log::trace};

use crate::rudder::{
    analysis::dfa::StatementUseAnalysis, Block, CastOperationKind, ConstantValue,
    PrimitiveTypeClass, StatementKind, Type,
};

pub fn run(f: crate::rudder::Function) -> bool {
    let mut changed = false;

    trace!("debundling {}", f.name());
    for block in f.entry_block().iter() {
        changed |= run_on_block(block);
    }

    changed
}

fn run_on_block(block: Block) -> bool {
    let mut changed = false;

    changed |= do_direct_debundle(&block);
    //changed |= transform_constant_length_bundles(&block);

    changed
}

fn do_direct_debundle(block: &Block) -> bool {
    let sua = StatementUseAnalysis::new(&block);

    let mut changed = false;

    let mut bundles = HashMap::default();
    for stmt in block.statements() {
        changed |= match stmt.kind() {
            StatementKind::Bundle { value, length } => {
                bundles.insert(stmt.clone(), (value.clone(), length.clone()));
                false
            }
            StatementKind::UnbundleValue { bundle } => {
                trace!("debundling unbundle-val on {}", bundle);

                let Some((live_value, _)) = bundles.get(&bundle) else {
                    // Need to ignore non-bundle statements (think about this)
                    continue;
                };

                if sua.is_dead(&stmt) {
                    panic!(
                        "dead unbundle-val that hasn't been eliminated: {} in block {}",
                        stmt, block
                    )
                }

                for use_ in sua.get_uses(&stmt) {
                    use_.replace_use(stmt.clone(), live_value.clone());
                }

                false
            }
            StatementKind::UnbundleLength { bundle } => {
                trace!("debundling unbundle-len on {}", bundle);

                let Some((_, live_length)) = bundles.get(&bundle) else {
                    // Need to ignore non-bundle statements (think about this)
                    continue;
                };

                for use_ in sua.get_uses(&stmt) {
                    use_.replace_use(stmt.clone(), live_length.clone());
                }

                false
            }
            _ => false,
        }
    }

    changed
}

fn _transform_constant_length_bundles(block: &Block) -> bool {
    let mut changed = false;

    for stmt in block.statements() {
        changed |= match stmt.kind() {
            StatementKind::Bundle { value, length } => {
                if let StatementKind::Constant {
                    typ: length_type,
                    value: ConstantValue::UnsignedInteger(target_length),
                } = length.kind()
                {
                    // we've got a bundle with a known length here
                    // can we replace it with a cast to the correct bit width?

                    let value_length = value.typ().width_bits();
                    let target_type = Rc::new(Type::new_primitive(
                        PrimitiveTypeClass::UnsignedInteger,
                        target_length,
                    ));

                    if target_length < value_length {
                        stmt.replace_kind(StatementKind::Cast {
                            kind: CastOperationKind::Truncate,
                            typ: target_type,
                            value: value.clone(),
                        });
                    } else if target_length > value_length {
                        stmt.replace_kind(StatementKind::Cast {
                            kind: CastOperationKind::ZeroExtend,
                            typ: target_type,
                            value: value.clone(),
                        });
                    } else {
                        stmt.replace_kind(StatementKind::Cast {
                            kind: CastOperationKind::Reinterpret,
                            typ: target_type,
                            value: value.clone(),
                        });
                    }

                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    changed
}