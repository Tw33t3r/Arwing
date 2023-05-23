import { createSignal, For } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";

import { createOptions } from "@thisbeyond/solid-select";

import { Select } from "./common-components/select";
import { characters } from "./consts/characters";

function App() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");
  const [player, setPlayer] = createSignal("");

  //TODO(Tweet): Here is where we will query arwing_core 
  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name: name() }));
  }

  const characterOptions = createOptions(characters.map(character => (character.name)));

  function characte(searchString: string) {

    setName(searchString)
  }

  return (
    <div class="bg-gray-100">
      < h1 class="mb-4 text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl" >
        Matchup
      </h1 >
      <div >
        <div>
          {/* TODO(Tweet): change the onchange to filter through characters from character enums */}
          <Select
            placeholder="Player"
            onChange={(e) => setPlayer(e.currentTarget.value)}
            {...characterOptions}
          />
          <Select
            placeholder="Opponent"
            {...characterOptions}
          />
          <button type="button" onClick={() => greet()}>
            Greet
          </button>
        </div>

      </div>

      <p>{greetMsg()}</p>
      <Select {...characterOptions} />
    </div >
  );
}

export default App;
