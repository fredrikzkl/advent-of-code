public class Node {
  public Device device;
  public Node? parent;
  public List<Node> children;

  public Node(Device device, Node parent = null){
    this.device = device;

    this.parent = parent;
    this.children = new List<Node>();
  }
}
