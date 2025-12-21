public enum SpruceType { DAC, FFT }

public class Spruce {
  SpruceType type;
  public Node origin;

  public List<Path> root_to_origin;
  public List<Path> origin_to_end;

  public Spruce(SpruceType type, Node origin) {
    this.type = type;
    this.origin = origin;

    this.root_to_origin = new List<Path>();
    this.origin_to_end = new List<Path>();
  }

  public void GrowDown(List<Device> devices) {
    Queue<Node> queue = new Queue<Node>();
    queue.Enqueue(origin);

    int depthCounter = 0;

    // Grow Down
    while (queue.Count > 0) {
      int nodesAtCurrentLevel = queue.Count;
      Console.WriteLine($"Depth: {depthCounter}, Queue Size: {queue.Count}");

      for (int i = 0; i < nodesAtCurrentLevel; i++) {
        Node current = queue.Dequeue();

        foreach (String output in current.device.outputs) {
          Device? matchingDevice = devices.FirstOrDefault(d => d.input.Contains(output));
          if (matchingDevice == null ) throw new Exception($"No matching device found for output: {output}");

          Node childNode = new Node(matchingDevice, current);


          if (childNode.device.IsOut()){
            Path newPath = new Path();
            newPath.ByParent(origin, childNode);
            origin_to_end.Add(newPath);
            Console.WriteLine($"{newPath}");
            continue;
          }

          // Loop checking:
          // Check path, for potential cycle
          if (childNode.device == this.origin.device) {
            Console.WriteLine("Origin Cycle detected, skipping...");
            continue;
          }

          Path pathToOrigin = new Path();
          pathToOrigin.ByParent(origin, childNode);
          bool hasCycle = false;
          foreach(Node node in pathToOrigin.nodes) {
            if (node != childNode && node.device.input == childNode.device.input) {
              Console.WriteLine("Other Cycle detected in path, skipping...");
              hasCycle = true;
              break;
            }
          }

          if (!hasCycle) {
            queue.Enqueue(childNode);
          }
        }
      }
      depthCounter++;
    }
  }

  // Not in use
  public void GrowUp(List<Device> devices) {
    Queue<Node> queue = new Queue<Node>();
    queue.Enqueue(origin);

    while(queue.Count > 0) {
      Node current = queue.Dequeue();

      List<Device> parents = devices.Where(d => d.outputs.Contains(current.device.input)).ToList();

      foreach(Device parentDevice in devices) {
        Node parentNode = new Node(parentDevice);
        parentNode.children.Add(current);

        if (parentNode.device.IsStart()) {
          // TODO: hardest shit i have ever made
          Console.WriteLine("Start hit");
          continue;
        }

        queue.Enqueue(parentNode);
      }
    }
  }
}
