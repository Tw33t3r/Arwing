import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/dialog';
import { appDataDir } from '@tauri-apps/api/path';

import { createOptions } from "@thisbeyond/solid-select";

import { Select } from "./common-components/select";
import { InternalCharacters, characters } from "./consts/characters";
import Interactions from "./fixed-components/interaction";

function App() {
  const [greetMsg, setGreetMsg] = createSignal("");
  //TODO(Tweet): don't just initialize to mario
  const [player, setPlayer] = createSignal(characters[0]);
  const [opponent, setOpponent] = createSignal(characters[0]);
  const [parseLocation, setParseLocation] = createSignal("");

  let interactionData: any;
  //TODO(Tweet): Setup store of interactions https://www.solidjs.com/tutorial/stores_createstore 

  //TODO(Tweet): Here is where we will query arwing_core 
  async function search() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    for (let interaction of interactionData) {
      console.log(interaction.characterId);
      console.log(interaction.moveId);
      console.log(interaction.withinFrames);
    }
  }

  async function openFolder() {
    // Open a selection dialog for directories
    const selected = await open({
      directory: true,
      multiple: true,
      defaultPath: await appDataDir(),
    });
    if (Array.isArray(selected)) {
      // TODO: search in multiple folders  
      setParseLocation(selected[0]);
    } else if (selected === null) {
      //TODO: Add error here
    } else {
      setParseLocation(selected);
    }
  }

  const characterOptions = createOptions(characters, { key: "name" });
  const formatCharacters = (item: any, type: any) => {
    return (type === "option" ? item.name : item.name);
  };

  return (
    <div class="bg-gray-100">
      < h1 class="mb-4 text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl" >
        Matchup
      </h1 >
      <div >
        <div class="w-full grid grid-cols-4">
          <Select
            placeholder="Player"
            onChange={(e) => {
              setPlayer(e)
            }
            }
            {...characterOptions}
          />
          <Select
            placeholder="Opponent"
            onChange={(e) =>
              setOpponent(e)
            }
            {...characterOptions}
          />
          <button
            type="button"
            class="text-white bg-gradient-to-r from-teal-400 via-teal-500 to-teal-600 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-teal-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center mr-2 mb-2"
            onClick={() => openFolder()}>
            SLP Folder
          </button>
        </div>
        <Interactions
          player={player}
          opponent={opponent}
          ref={interactionData}
        />
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
