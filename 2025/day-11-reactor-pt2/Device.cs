public class Device {
  public String input;
  public String[] outputs;

  public Device(String line) {
    String[] split = line.Split(":");
    String[] outputs = split[1].Split(" ").Select(s => s.Trim()).Where(s => !string.IsNullOrWhiteSpace(s)).ToArray();

    this.input = split[0].Trim();
    this.outputs = outputs;
  }

  public bool IsOut() => outputs.Length == 1 && outputs[0] == "out";

  public bool IsStart() => input == "svr";

  public bool IsDac() => input == "dac";

  public bool IsFFT() => input == "fft";

  public override string ToString() => $"Device: input: '{this.input}', outputs: [{string.Join(", ", this.outputs)}]";
}
