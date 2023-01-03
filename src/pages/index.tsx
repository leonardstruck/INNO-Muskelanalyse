import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Command } from "@tauri-apps/api/shell"
import Image from "next/image";
import TechnikumLogo from "../assets/technikum.svg";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const command = Command.sidecar("../resources/dist/test", [name]);
    const result = await command.execute();
    setGreetMsg(result.stdout);
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <div>
          <input
            id="greet-input"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a name..."
          />
          <button type="button" onClick={() => greet()}>
            Greet
          </button>
        </div>
      </div>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
