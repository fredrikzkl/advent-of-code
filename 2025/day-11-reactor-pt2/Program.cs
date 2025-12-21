var inputPath = System.IO.Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "data", "input.txt");
var input = File.ReadAllText(inputPath).Trim();

List<Device> devices = new List<Device>();

foreach (String line in File.ReadLines(inputPath)) {
  Device device = new Device(line);
  devices.Add(device);
  Console.WriteLine(device);
}

List<Device> dacDevices = devices.Where(d => d.IsDac()).ToList();
List<Device> fftDevices = devices.Where(d => d.IsFFT()).ToList();

List<Path> finalPaths = new List<Path>();

foreach (Device dacDevice in dacDevices) {
  Node dacNode = new Node(dacDevice);
  Spruce dacSpruce = new Spruce(SpruceType.DAC, dacNode);

  Console.WriteLine("DAC Grow down...");
  dacSpruce.GrowDown(devices);

  Console.WriteLine("Growing complete");

  foreach (var path in dacSpruce.origin_to_end) {
    if (path.IsValid() && !ContainsPath(finalPaths, path)) {
      finalPaths.Add(path);
      Console.WriteLine(path);
    }
  }
}

foreach (Device fftDevice in fftDevices) {
  Node fftNode = new Node(fftDevice);
  Spruce fftSpruce = new Spruce(SpruceType.FFT, fftNode);

  Console.WriteLine("FFT Grow down...");
  fftSpruce.GrowDown(devices);
  Console.WriteLine("Growing complete");

  foreach (var path in fftSpruce.origin_to_end) {
    if (path.IsValid() && !ContainsPath(finalPaths, path)) {
      finalPaths.Add(path);
      Console.WriteLine(path);
    }
  }
}

Console.WriteLine($"Total valid paths found: {finalPaths.Count}");

static bool ContainsPath(List<Path> paths, Path targetPath) {
  foreach (var path in paths) {
    if (path.Equals(targetPath)) {
      return true;
    }
  }
  return false;
}


