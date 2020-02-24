use cgns::*;
use static_assertions::*;

assert_trait_super_all!(
    Node: GotoTarget,
    RwNode<'static>,
    ChildNode<'static>,
    BaseRefNode,
    OnlyChildNode<'static>,
    IndexableNode,
    IterableNode<'static>,
    SiblingNode<'static>,
    // ParentNode<'static>
);

assert_impl_all!(Library: Drop);
assert_not_impl_any!(Library: Sync, Send, Node);

assert_not_impl_any!(
    File: Sync,
    Send,
    GotoTarget,
    RwNode<'static>,
    ChildNode<'static>,
    BaseRefNode,
    OnlyChildNode<'static>,
    SiblingNode<'static>,
    IterableNode<'static>,
);
assert_impl_all!(File: IndexableNode, ParentNode<'static, Base<'static>>);

assert_not_impl_any!(Base: Sync, Send, OnlyChildNode<'static>,);
assert_impl_all!(
    Base: GotoTarget,
    RwNode<'static>,
    ChildNode<'static>,
    BaseRefNode,
    IndexableNode,
    IterableNode<'static>,
    SiblingNode<'static>,
    ParentNode<'static, Zone<'static>>,
    ParentNode<'static, SimulationType<'static>>
);

assert_not_impl_any!(Zone: Sync, Send, OnlyChildNode<'static>,);

assert_impl_all!(
    Zone: GotoTarget,
    RwNode<'static>,
    ChildNode<'static>,
    BaseRefNode,
    IndexableNode,
    IterableNode<'static>,
    SiblingNode<'static>,
    // ParentNode<'static, > //TODO
);

assert_not_impl_any!(
    SimulationType: Sync,
    Send,
    SiblingNode<'static>,
    IndexableNode,
    IterableNode<'static>,
);
assert_impl_all!(
    SimulationType: GotoTarget,
    RwNode<'static>,
    ChildNode<'static>,
    BaseRefNode,
    OnlyChildNode<'static>,
);

assert_not_impl_any!(Ordinal<Base>: Sync, Send, IndexableNode, IterableNode<'static>, SiblingNode<'static>);
assert_impl_all!(
    Ordinal<Base>: GotoTarget,
    RwNode<'static>,
    ChildNode<'static>,
    BaseRefNode,
    OnlyChildNode<'static>,
);

assert_not_impl_any!(Ordinal<Zone>: Sync, Send, IndexableNode, IterableNode<'static>, SiblingNode<'static>);
assert_impl_all!(
    Ordinal<Zone>: GotoTarget,
    RwNode<'static>,
    ChildNode<'static>,
    BaseRefNode,
    OnlyChildNode<'static>,
);

assert_not_impl_any!(Ordinal<SimulationType>: Sync, Send, IndexableNode, IterableNode<'static>, SiblingNode<'static>);
assert_impl_all!(
    Ordinal<SimulationType>: GotoTarget,
    RwNode<'static>,
    ChildNode<'static>,
    BaseRefNode,
    OnlyChildNode<'static>,
);

assert_not_impl_any!(Descriptor<Base>: Sync, Send, OnlyChildNode<'static>,);
assert_impl_all!(Descriptor<Base>:  GotoTarget,
    RwNode<'static>,
    ChildNode<'static>,
    BaseRefNode,
    IndexableNode,
    IterableNode<'static>,
    SiblingNode<'static>,
);

assert_not_impl_any!(Descriptor<Zone>: Sync, Send, OnlyChildNode<'static>,);
assert_impl_all!(Descriptor<Zone>:  GotoTarget,
    RwNode<'static>,
    ChildNode<'static>,
    BaseRefNode,
    IndexableNode,
    IterableNode<'static>,
    SiblingNode<'static>,
);

assert_not_impl_any!(Descriptor<SimulationType>: Sync, Send, OnlyChildNode<'static>,);
assert_impl_all!(Descriptor<SimulationType>:  GotoTarget,
    RwNode<'static>,
    ChildNode<'static>,
    BaseRefNode,
    IndexableNode,
    IterableNode<'static>,
    SiblingNode<'static>,
);
