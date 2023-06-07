import { createSignal, For } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/dialog';
import { appDataDir } from '@tauri-apps/api/path';

import { createOptions } from "@thisbeyond/solid-select";

import { Select } from "./common-components/select";
import { InternalCharacters, characters } from "./consts/characters";

function App() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [playerId, setPlayerId] = createSignal(0);
  const [opponentId, setOpponentId] = createSignal(0);
  const [parseLocation, setParseLocation] = createSignal("");

  //TODO(Tweet): Here is where we will query arwing_core 
  async function search() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    console.log(opponentId());
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

  function setInteractionFrom(from: String) {
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
            onChange={(e) =>
              setPlayerId(parseInt(InternalCharacters[e]))
            }
            {...characterOptions}
          />
          <Select
            placeholder="Opponent"
            onChange={(e) =>
              setOpponentId(parseInt(InternalCharacters[e]))
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
        <h2 class="mb-4 text-3xl font-extrabold leading-none tracking-tight text-gray-900 md:text-4xl lg:text-5xl">
          Interaction
        </h2>
        <div class="w-full grid grid-cols-3">
          <Select
            placeholder="Character"
            onChange={(e) =>
              setInteractionFrom(e)
            }
            {...createOptions([characters[playerId()].name, characters[opponentId()].name])}
          />
          <Select
            placeholder="Move"
            onChange={(e) =>
              setInteractionFrom(e)
            }
            //TODO(Tweet): Refactor Interactions and interactionRows to have state on each row for selected character
            {...createOptions(characters[playerId()].moves.map(move => move.moveName))}
          />
          <div class="col-span-1 relative">
            <input type="text" id="floating_outlined" class="block py-3 px-2 w-full border border-gray-200 rounded leading-normal focus:outline-none focus:ring-4 focus:border-blue-600 peer" placeholder=" " />
            <label for="floating_outlined" class="absolute text-sm text-gray-500 duration-300 transform -translate-y-4 scale-75 top-2 z-10 origin-[0] bg-white px-2 peer-focus:px-2 peer-focus:text-blue-600 peer-placeholder-shown:scale-100 peer-placeholder-shown:-translate-y-1/2 peer-placeholder-shown:top-1/2 peer-focus:top-2 peer-focus:scale-75 peer-focus:-translate-y-4 left-1">Frames Until This Move</label>
          </div>
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
