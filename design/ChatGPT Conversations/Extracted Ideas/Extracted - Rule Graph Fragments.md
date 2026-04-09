# Links

### Part 2
1. [[Part 2 Raw#Please Do [Show Rule Graph Fragments]|Rule graph fragment introduction]]

### Part 3
1. [[Part 3 Raw#8. Barkskin Fragment|Barkskin example]] (see sections above for more barkskin)
2. [[Part 3 Raw#13. Data Loader|Inventory rule fragment loaders]]

# Discussion

It's unclear to me how rule fragments differ from just giving the graph a `Vec<Node>` or `Vec<RuleNode>` to add to its collection.

# Extracted Data Structures and Representations

### Part 2

#### #1
```Rust
pub struct RuleFragment {  
    pub nodes: Vec<Node>,  
    pub constants: Vec<ConstantNode>,  
    pub contributions: Vec<Contribution>,  
}

trait RuleSource {  
    fn fragments(&self) -> Vec<RuleFragment>;  
}

pub struct RuleEngine {  
    pub graph: RuleGraph,  
}

// example of a ConditionalFragment
ConditionalFragment {  
    condition: Condition::Raging,  
    fragment: RuleFragment { ... }  
}
```

# (Pseudo)Code

### Part 2

#### #1
Example: Ring of Protection
```
Constant Node:  
ring_protection_bonus = 1  
  
Contribution:  
ac_bonus += ring_protection_bonus  
saving_throw_bonus += ring_protection_bonus
```
```Rust
RuleFragment {  
    constants: vec![  
        ConstantNode {  
            id: NodeId("ring_protection_bonus"),  
            value: Value::Int(1),  
        }  
    ],  
  
    contributions: vec![  
        Contribution {  
            target: NodeId("ac_bonus"),  
            source: NodeId("ring_protection_bonus"),  
        },  
        Contribution {  
            target: NodeId("saving_throw_bonus"),  
            source: NodeId("ring_protection_bonus"),  
        }  
    ],  
  
    nodes: vec![],  
}
```

Example: Barbarian Unarmored Defense
```Rust
RuleFragment {  
    nodes: vec![  
        Node {  
            id: NodeId("unarmored_formula"),  
            value_type: ValueType::Int,  
            stack: StackRule::Override,  
            deps: vec![  
                NodeId("dex_mod"),  
                NodeId("con_mod"),  
            ],  
        }  
    ],  
  
    contributions: vec![  
        Contribution {  
            target: NodeId("ac_formula"),  
            source: NodeId("unarmored_formula"),  
        }  
    ],  
  
    constants: vec![],  
}
```

Inventory `trait RuleSource` implementation
```Rust
impl Inventory {  
    pub fn fragments(&self, db: &ItemDb) -> Vec<RuleFragment> {  
        self.items  
            .iter()  
            .filter(|i| i.equipped)  
            .flat_map(|item| db.get(item.id).fragments())  
            .collect()  
    }  
}
```

Fragment merger
```Rust
fn merge_fragment(graph: &mut RuleGraph, frag: RuleFragment) {  
    for node in frag.nodes {  
        graph.add_node(node);  
    }  
  
    for constant in frag.constants {  
        graph.add_constant(constant);  
    }  
  
    for contribution in frag.contributions {  
        graph.add_contribution(contribution);  
    }  
}
```

### Part 3

#### #1
```Rust
RuleFragment {  
    constants: vec![  
        ConstantNode {  
            id: NodeId("barkskin_min_ac"),  
            value: Value::Int(16),  
        }  
    ],  
  
    contributions: vec![  
        Contribution {  
            target: NodeId("ac_min"),  
            source: NodeId("barkskin_min_ac"),  
        }  
    ],  
  
    nodes: vec![]  
}
```

#### #2
```Rust
pub fn load_item_fragment(data: ItemData) -> RuleFragment {  
    RuleFragment {  
        constants: vec![  
            ConstantNode {  
                id: NodeId("ring_ac_bonus"),  
                value: Value::Int(1),  
            }  
        ],  
        contributions: vec![  
            Contribution {  
                target: NodeId("ac_bonus"),  
                source: NodeId("ring_ac_bonus"),  
            }  
        ],  
        nodes: vec![],  
    }  
}

impl Inventory {  
    pub fn fragments(&self) -> Vec<RuleFragment> {  
        self.items  
            .iter()  
            .filter(|i| i.equipped)  
            .flat_map(|i| i.fragment.clone())  
            .collect()  
    }  
}
```

