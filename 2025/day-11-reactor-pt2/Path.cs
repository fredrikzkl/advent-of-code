public class Path {
  public List<Node> nodes;

  private bool dac = false;
  private bool fft = false;

  public Path() {
    this.nodes = new List<Node>();
  }

  // ByParent constructs path from end, then traverse by parent up to start
  public void ByParent(Node start, Node end) {
    Node current = end;
    List<Node> tempNodes = new List<Node>();

    while (current != null) {
      tempNodes.Add(current);

      if (current.device.IsDac()) {
        this.dac = true;
      }

      if (current.device.IsFFT()) {
        this.fft = true;
      }

      if (current == start) break;
      current = current.parent;
    }

    tempNodes.Reverse();
    this.nodes = tempNodes;
  }

  public bool IsValid() => this.dac && this.fft;

  public bool ContainsDevice(Node node) {
    return this.nodes.Where(n => n.device.input == node.device.input).Any();
  }

  public override string ToString() {
    return $"Path: [{string.Join(" -> ", this.nodes.Select(n => n.device.input))}], DAC: {this.dac}, FFT: {this.fft}";
  }

  public bool Equals(Path other) {
    if (other == null) return false;
    if (this.nodes.Count != other.nodes.Count) return false;

    for (int i = 0; i < this.nodes.Count; i++) {
      if (this.nodes[i].device.input != other.nodes[i].device.input) {
        return false;
      }
    }

    return true;
  }
}
