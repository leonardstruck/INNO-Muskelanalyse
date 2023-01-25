import { Command } from "@tauri-apps/api/shell"

async function handleButtonClick() {
  const command = Command.sidecar("resources/segmentation/bin/segmentation", ["/Users/leonardstruck/Downloads/Input/LORU.png", "/Users/leonardstruck/Downloads/Input/out"]);
  const output = await command.execute();

  console.log(JSON.parse(output.stdout));
}

function App() {
  return (
    <>
      <button onClick={handleButtonClick}>Click me</button>
    </>
  );
}

export default App;
