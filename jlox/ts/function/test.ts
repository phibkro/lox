type State = {};
type Action = {};
type Node = {
  parent?: Node;
  state: State;
  action: Action;
};

function createNode(state: State, action: Action, parent?: Node) {
  const node: Node = {
    state,
    action,
    parent,
  };
  return node;
}
