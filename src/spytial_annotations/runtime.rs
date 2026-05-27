use std::sync::LazyLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

/// Wire-format helpers for the `negated` flag on constraints.
///
/// `spytial-core` represents constraint negation as `hold: never` inside the
/// inner constraint object (a positive constraint omits the `hold` key
/// entirely). On the Rust side we keep an ergonomic `negated: bool`, and
/// these helpers translate to/from the `hold` string at serialization
/// boundaries.
mod hold_serde {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(negated: &bool, ser: S) -> Result<S::Ok, S::Error> {
        // Caller is `skip_serializing_if = "is_not_negated"` — this only runs
        // when `*negated == true`, so always emit "never".
        debug_assert!(*negated);
        ser.serialize_str("never")
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
        let s: Option<String> = Option::deserialize(d)?;
        Ok(s.as_deref() == Some("never"))
    }
}

fn is_not_negated(negated: &bool) -> bool {
    !*negated
}

/// Main structure containing all SpyTial decorators for a type or instance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpytialDecorators {
    pub constraints: Vec<Constraint>,
    pub directives: Vec<Directive>,
}

impl Default for SpytialDecorators {
    fn default() -> Self {
        Self {
            constraints: Vec::new(),
            directives: Vec::new(),
        }
    }
}

/// Constraint types (layout/structural constraints)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Constraint {
    Orientation(OrientationConstraint),
    Align(AlignConstraint),
    Cyclic(CyclicConstraint),
    Group(GroupConstraint),
}

/// Directive types (visual/behavioral directives)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Directive {
    AtomColor(AtomColorDirective),
    Size(SizeDirective),
    Icon(IconDirective),
    EdgeStyle(EdgeStyleDirective),
    Projection(ProjectionDirective),
    Attribute(AttributeDirective),
    HideField(HideFieldDirective),
    HideAtom(HideAtomDirective),
    InferredEdge(InferredEdgeDirective),
    Tag(TagDirective),
    Flag(FlagDirective),
}

// Constraint implementations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrientationConstraint {
    pub orientation: OrientationParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrientationParams {
    pub selector: String,
    pub directions: Vec<String>,
    #[serde(
        rename = "hold",
        default,
        skip_serializing_if = "is_not_negated",
        serialize_with = "hold_serde::serialize",
        deserialize_with = "hold_serde::deserialize"
    )]
    pub negated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AlignConstraint {
    pub align: AlignParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AlignParams {
    pub selector: String,
    pub direction: String,
    #[serde(
        rename = "hold",
        default,
        skip_serializing_if = "is_not_negated",
        serialize_with = "hold_serde::serialize",
        deserialize_with = "hold_serde::deserialize"
    )]
    pub negated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CyclicConstraint {
    pub cyclic: CyclicParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CyclicParams {
    pub selector: String,
    pub direction: String,
    #[serde(
        rename = "hold",
        default,
        skip_serializing_if = "is_not_negated",
        serialize_with = "hold_serde::serialize",
        deserialize_with = "hold_serde::deserialize"
    )]
    pub negated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupConstraint {
    pub group: GroupParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GroupParams {
    FieldBased {
        field: String,
        #[serde(rename = "groupOn")]
        group_on: u32,
        #[serde(rename = "addToGroup")]
        add_to_group: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        selector: Option<String>,
        #[serde(
            rename = "hold",
            default,
            skip_serializing_if = "is_not_negated",
            serialize_with = "hold_serde::serialize",
            deserialize_with = "hold_serde::deserialize"
        )]
        negated: bool,
    },
    SelectorBased {
        selector: String,
        name: String,
        #[serde(
            rename = "hold",
            default,
            skip_serializing_if = "is_not_negated",
            serialize_with = "hold_serde::serialize",
            deserialize_with = "hold_serde::deserialize"
        )]
        negated: bool,
    },
}

// Directive implementations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AtomColorDirective {
    #[serde(rename = "atomColor")]
    pub atom_color: AtomColorParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AtomColorParams {
    pub selector: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SizeDirective {
    pub size: SizeParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SizeParams {
    pub selector: String,
    pub height: u32,
    pub width: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IconDirective {
    pub icon: IconParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IconParams {
    pub selector: String,
    pub path: String,
    #[serde(rename = "showLabels")]
    pub show_labels: bool,
}

/// `EdgeStyleDirective` is the canonical edge-styling directive — color,
/// line style, weight, label visibility, and edge visibility in one
/// directive. Mirrors `EdgeStyleDirective` in `spytial-core`'s
/// `src/layout/layoutspec.ts`.
///
/// The wire-format YAML key is `edgeColor:` (kept for backwards
/// compatibility with `spytial-core`'s parser, where `EdgeColorDirective`
/// is a type alias for `EdgeStyleDirective`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EdgeStyleDirective {
    #[serde(rename = "edgeColor")]
    pub edge_style: EdgeStyleParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EdgeStyleParams {
    pub field: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showLabel")]
    pub show_label: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectionDirective {
    pub projection: ProjectionParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectionParams {
    pub sig: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttributeDirective {
    pub attribute: AttributeParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttributeParams {
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HideFieldDirective {
    #[serde(rename = "hideField")]
    pub hide_field: HideFieldParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HideFieldParams {
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HideAtomDirective {
    #[serde(rename = "hideAtom")]
    pub hide_atom: HideAtomParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HideAtomParams {
    pub selector: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InferredEdgeDirective {
    #[serde(rename = "inferredEdge")]
    pub inferred_edge: InferredEdgeParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InferredEdgeParams {
    pub name: String,
    pub selector: String,
}

/// `TagDirective` adds computed attributes to nodes based on n-ary selector
/// evaluation. Mirrors `TagDirective` in `spytial-core`'s
/// `src/layout/layoutspec.ts` — the canonical YAML form is:
///
/// ```yaml
/// directives:
///   - tag:
///       toTag: 'Person'
///       name: 'status'
///       value: 'Person.status'
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TagDirective {
    pub tag: TagParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TagParams {
    #[serde(rename = "toTag")]
    pub to_tag: String,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FlagDirective {
    pub flag: String,
}

/// Trait implemented by structs with SpyTial decorators.
///
/// Types that `#[derive(SpytialDecorators)]` get an implementation of this
/// trait whose `decorators()` method returns the full set of constraints and
/// directives declared via attributes on the type and its nested field types.
pub trait HasSpytialDecorators {
    fn decorators() -> SpytialDecorators;
}

impl<T: HasSpytialDecorators + ?Sized> HasSpytialDecorators for &T {
    fn decorators() -> SpytialDecorators {
        T::decorators()
    }
}

// ── Probe mechanism for safe compile-time decorator collection ──────────
//
// Problem: the derive macro needs to collect decorators from field types,
// but it can't tell at macro-expansion time whether a type implements
// `HasSpytialDecorators`.  Generating `.include_decorators_from_type::<T>()`
// for a type that *doesn't* implement the trait would be a compile error.
//
// Solution: a zero-cost probe that resolves at the call-site (where the
// concrete type is known).  Inherent methods always beat trait methods in
// Rust's method resolution, so:
//
//   - If `T: HasSpytialDecorators` → the inherent `get()` is chosen → real
//     decorators.
//   - Otherwise → `DefaultDecorators::get()` is chosen → empty decorators.
//

/// Zero-sized probe used by macro-generated code to safely collect
/// decorators from a type that may or may not implement
/// [`HasSpytialDecorators`].
pub struct DecoProbe<T>(pub ::std::marker::PhantomData<T>);

/// Inherent impl – available only when `T` has the derive.
/// Because inherent methods take priority over trait methods, this is
/// chosen whenever it exists.
impl<T: HasSpytialDecorators> DecoProbe<T> {
    pub fn get(self) -> SpytialDecorators {
        T::decorators()
    }
}

/// Blanket fallback – available for *every* `T`.  Chosen only when the
/// inherent `get` above does not exist (i.e. `T` does not implement
/// `HasSpytialDecorators`).
pub trait DefaultDecorators {
    fn get(self) -> SpytialDecorators;
}

impl<T> DefaultDecorators for DecoProbe<T> {
    fn get(self) -> SpytialDecorators {
        SpytialDecorators::default()
    }
}

/// Global registry for type-level decorators keyed by type name
static TYPE_REGISTRY: LazyLock<Mutex<HashMap<String, SpytialDecorators>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Register SpyTial decorators for a type (used by procedural macros)
pub fn register_type_decorators(type_name: &str, decorators: SpytialDecorators) {
    let mut registry = TYPE_REGISTRY.lock().unwrap();
    registry.insert(type_name.to_string(), decorators);
}

/// Get SpyTial decorators for a type by name
/// Get decorators for a specific type, if registered
pub fn get_type_decorators(type_name: &str) -> Option<SpytialDecorators> {
    let registry = TYPE_REGISTRY.lock().unwrap();
    registry.get(type_name).cloned()
}

/// Serialize decorators to YAML string
pub fn to_yaml(decorators: &SpytialDecorators) -> Result<String, serde_yml::Error> {
    serde_yml::to_string(decorators)
}

/// Builder for constructing spatial decorators
#[derive(Debug)]
pub struct SpytialDecoratorsBuilder {
    constraints: Vec<Constraint>,
    directives: Vec<Directive>,
}

impl SpytialDecoratorsBuilder {
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            directives: Vec::new(),
        }
    }

    pub fn orientation(mut self, selector: &str, directions: Vec<&str>, negated: bool) -> Self {
        self.constraints
            .push(Constraint::Orientation(OrientationConstraint {
                orientation: OrientationParams {
                    selector: selector.to_string(),
                    directions: directions.iter().map(|s| s.to_string()).collect(),
                    negated,
                },
            }));
        self
    }

    pub fn align(mut self, selector: &str, direction: &str, negated: bool) -> Self {
        self.constraints.push(Constraint::Align(AlignConstraint {
            align: AlignParams {
                selector: selector.to_string(),
                direction: direction.to_string(),
                negated,
            },
        }));
        self
    }

    pub fn cyclic(mut self, selector: &str, direction: &str, negated: bool) -> Self {
        self.constraints.push(Constraint::Cyclic(CyclicConstraint {
            cyclic: CyclicParams {
                selector: selector.to_string(),
                direction: direction.to_string(),
                negated,
            },
        }));
        self
    }

    pub fn group_field_based(
        mut self,
        field: &str,
        group_on: u32,
        add_to_group: u32,
        selector: Option<&str>,
        negated: bool,
    ) -> Self {
        self.constraints.push(Constraint::Group(GroupConstraint {
            group: GroupParams::FieldBased {
                field: field.to_string(),
                group_on,
                add_to_group,
                selector: selector.map(|s| s.to_string()),
                negated,
            },
        }));
        self
    }

    pub fn group_selector_based(mut self, selector: &str, name: &str, negated: bool) -> Self {
        self.constraints.push(Constraint::Group(GroupConstraint {
            group: GroupParams::SelectorBased {
                selector: selector.to_string(),
                name: name.to_string(),
                negated,
            },
        }));
        self
    }

    pub fn atom_color(mut self, selector: &str, value: &str) -> Self {
        self.directives
            .push(Directive::AtomColor(AtomColorDirective {
                atom_color: AtomColorParams {
                    selector: selector.to_string(),
                    value: value.to_string(),
                },
            }));
        self
    }

    pub fn size(mut self, selector: &str, height: u32, width: u32) -> Self {
        self.directives.push(Directive::Size(SizeDirective {
            size: SizeParams {
                selector: selector.to_string(),
                height,
                width,
            },
        }));
        self
    }

    pub fn icon(mut self, selector: &str, path: &str, show_labels: bool) -> Self {
        self.directives.push(Directive::Icon(IconDirective {
            icon: IconParams {
                selector: selector.to_string(),
                path: path.to_string(),
                show_labels,
            },
        }));
        self
    }

    #[allow(clippy::too_many_arguments)]
    pub fn edge_style(
        mut self,
        field: &str,
        value: &str,
        selector: Option<&str>,
        filter: Option<&str>,
        style: Option<&str>,
        weight: Option<f64>,
        show_label: Option<bool>,
        hidden: Option<bool>,
    ) -> Self {
        self.directives
            .push(Directive::EdgeStyle(EdgeStyleDirective {
                edge_style: EdgeStyleParams {
                    field: field.to_string(),
                    value: value.to_string(),
                    selector: selector.map(|s| s.to_string()),
                    filter: filter.map(|s| s.to_string()),
                    style: style.map(|s| s.to_string()),
                    weight,
                    show_label,
                    hidden,
                },
            }));
        self
    }

    pub fn projection(mut self, sig: &str) -> Self {
        self.directives
            .push(Directive::Projection(ProjectionDirective {
                projection: ProjectionParams {
                    sig: sig.to_string(),
                },
            }));
        self
    }

    pub fn attribute(mut self, field: &str, selector: Option<&str>) -> Self {
        self.directives
            .push(Directive::Attribute(AttributeDirective {
                attribute: AttributeParams {
                    field: field.to_string(),
                    selector: selector.map(|s| s.to_string()),
                },
            }));
        self
    }

    pub fn hide_field(mut self, field: &str, selector: Option<&str>) -> Self {
        self.directives
            .push(Directive::HideField(HideFieldDirective {
                hide_field: HideFieldParams {
                    field: field.to_string(),
                    selector: selector.map(|s| s.to_string()),
                },
            }));
        self
    }

    pub fn hide_atom(mut self, selector: &str) -> Self {
        self.directives.push(Directive::HideAtom(HideAtomDirective {
            hide_atom: HideAtomParams {
                selector: selector.to_string(),
            },
        }));
        self
    }

    pub fn inferred_edge(mut self, name: &str, selector: &str) -> Self {
        self.directives
            .push(Directive::InferredEdge(InferredEdgeDirective {
                inferred_edge: InferredEdgeParams {
                    name: name.to_string(),
                    selector: selector.to_string(),
                },
            }));
        self
    }

    pub fn flag(mut self, name: &str) -> Self {
        self.directives.push(Directive::Flag(FlagDirective {
            flag: name.to_string(),
        }));
        self
    }

    pub fn tag(mut self, to_tag: &str, name: &str, value: &str) -> Self {
        self.directives.push(Directive::Tag(TagDirective {
            tag: TagParams {
                to_tag: to_tag.to_string(),
                name: name.to_string(),
                value: value.to_string(),
            },
        }));
        self
    }

    /// Include decorators from another type that implements HasSpytialDecorators.
    pub fn include_decorators_from_type<T: HasSpytialDecorators>(mut self) -> Self {
        let other_decorators = T::decorators();
        self.constraints.extend(other_decorators.constraints);
        self.directives.extend(other_decorators.directives);
        self
    }

    /// Merge another set of decorators into this builder.
    ///
    /// Used by the derive macro together with [`DecoProbe`] for safe
    /// compile-time decorator collection from field types that may or may
    /// not implement [`HasSpytialDecorators`].
    pub fn extend_with(mut self, other: SpytialDecorators) -> Self {
        self.constraints.extend(other.constraints);
        self.directives.extend(other.directives);
        self
    }

    pub fn build(self) -> SpytialDecorators {
        SpytialDecorators {
            constraints: self.constraints,
            directives: self.directives,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spytial_decorators_default() {
        let decorators = SpytialDecorators::default();
        assert!(decorators.constraints.is_empty());
        assert!(decorators.directives.is_empty());
    }

    #[test]
    fn test_yaml_serialization() {
        let decorators = SpytialDecorators {
            constraints: vec![
                Constraint::Orientation(OrientationConstraint {
                    orientation: OrientationParams {
                        selector: "value".to_string(),
                        directions: vec!["above".to_string()],
                        negated: false,
                    },
                }),
                Constraint::Align(AlignConstraint {
                    align: AlignParams {
                        selector: "siblings".to_string(),
                        direction: "horizontal".to_string(),
                        negated: false,
                    },
                }),
            ],
            directives: vec![Directive::Flag(FlagDirective {
                flag: "test_flag".to_string(),
            })],
        };

        let yaml = to_yaml(&decorators).unwrap();
        assert!(yaml.contains("orientation"));
        assert!(yaml.contains("align"));
        assert!(yaml.contains("flag"));
    }

}
