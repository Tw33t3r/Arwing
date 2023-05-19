import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");

  //TODO(Tweet): Here is where we will query arwing_core 
  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name: name() }));
  }

  return (
    <div class="bg-gray-100">
      < h1 class="mb-4 text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl" >
        Matchup
      </h1 >
      <div >
        <div>
          {/* TODO(Tweet): change the onchange to filter through characters from character enums */}
          <input
            class="block w-full p-4 pl-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500"
            id="greet-input"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Player"
          />
          <input
            class="block w-full p-4 pl-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500"
            id="greet-input"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Opponent"
          />

          <button type="button" onClick={() => greet()}>
            Greet
          </button>
        </div>

      </div>

      <p>{greetMsg()}</p>
    </div >
  );
}

export default App;
