import { createSignal, For } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";

import { createOptions } from "@thisbeyond/solid-select";

import { Select } from "./common-components/select";
import { characters } from "./consts/characters";

function App() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [player, setPlayer] = createSignal("");
  const [opponent, setOpponent] = createSignal("");

  //TODO(Tweet): Here is where we will query arwing_core 
  async function search() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    console.log(opponent());
  }

  const characterOptions = createOptions(characters.map(character => (character.name)));

  return (
    <div class="bg-gray-100">
      < h1 class="mb-4 text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl" >
        Matchup
      </h1 >
      <div >
        <div class="w-full grid grid-cols-4">
          {/* TODO(Tweet): change the onchange to filter through characters from character enums */}
          <Select
            placeholder="Player"
            onChange={(e) => setPlayer(e)}
            {...characterOptions}
          />
          <Select
            placeholder="Opponent"
            onChange={(e) =>
              setOpponent(e)
            }
            {...characterOptions}
          />
        </div>

        <button
          type="button"
          class="text-white bg-gradient-to-r from-teal-400 via-teal-500 to-teal-600 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-teal-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center mr-2 mb-2"
          onClick={() => search()}>
          Search
        </button>
      </div>

      <p>{greetMsg()}</p>
    </div >
  );
}

export default App;
