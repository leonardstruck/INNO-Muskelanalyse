import { Command } from "@tauri-apps/api/shell"

async function handleButtonClick() {
  const command = Command.sidecar("resources/segmentation/bin/segmentation");
  const output = await command.execute();

  console.log(output);
}

function App() {
  return (
    <>
      <button onClick={handleButtonClick}>Click me</button>
    </>
  );
}

export default App;
