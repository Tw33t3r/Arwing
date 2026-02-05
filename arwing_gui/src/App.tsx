import { createSignal, Show } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from '@tauri-apps/plugin-dialog';
import { BaseDirectory } from '@tauri-apps/plugin-fs';

import { createOptions } from "@thisbeyond/solid-select";

import { Select } from "./common-components/select";
import { InternalCharacters, characters } from "./consts/characters";
import Interactions from "./fixed-components/interaction";

function App() {
  //TODO(Tweet): don't just initialize to mario
  const [player, setPlayer] = createSignal(characters[0]);
  const [opponent, setOpponent] = createSignal(characters[0]);
  const [parseLocation, setParseLocation] = createSignal("");
  const [discoveredInteractions, setDiscoveredInteractions] = createSignal();

  let interactionData: any;
  //TODO(Tweet): Setup store of interactions https://www.solidjs.com/tutorial/stores_createstore 

  async function search() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    invoke('scan_for_interactions', {
      pathString: parseLocation(),
      player: player().internalId,
      opponent: opponent().internalId,
      interactions: interactionData
    })
      .then((message) => {
        console.log(message)
        setDiscoveredInteractions(message);
      })
      .catch((error) => console.error(error));
  }

  async function exportToJson() {
    const filePath = await save({
      filters: [{
        name: 'json',
        extensions: ['json'],
      }]
    });
    invoke('export_to_json', {
      exportLocation: filePath,
      parsedGames: discoveredInteractions()
    })
      .then((message) => console.log(message))
      .catch((error) => console.error(error));
  }

  async function openFolder() {
    // Open a selection dialog for directories
    const selected = await open({
      directory: true,
      multiple: true,
      defaultPath: BaseDirectory.AppLocalData,
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
    <div class="bg-gray-100 min-h-screen py-14 px-14">
      <div >
        < h1 class="mb-4 text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl" >
          Matchup
        </h1 >
        <div class="w-full grid grid-cols-4">
          <Select
            placeholder="Player"
            class="m-2"
            onChange={(e) => {
              setPlayer(e)
            }
            }
            {...characterOptions}
          />
          <Select
            placeholder="Opponent"
            class="m-2"
            onChange={(e) =>
              setOpponent(e)
            }
            {...characterOptions}
          />
          <div class="flex items-center">
            <button
              type="button"
              class="text-white bg-gradient-to-r from-teal-400 via-teal-500 to-teal-600 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-teal-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center"
              onClick={() => openFolder()}>
              SLP Folder
            </button>
            <div class="text-lg tracking-tight text-gray-800 lg:text-lg mx-4"> {parseLocation()} </div>
          </div>
        </div>
        <Interactions
          player={player}
          opponent={opponent}
          ref={interactionData}
        />
        <button
          type="button"
          class="text-white bg-gradient-to-r from-teal-400 via-teal-500 to-teal-600 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-teal-300 rounded-lg font-bold text-md px-5 py-2.5 text-center my-6"
          onClick={() => search()}>
          Search
        </button>
      </div>
      <div class="my-6">
        <Show
          when={discoveredInteractions()}
        >
          {discoveredInteractions && <p> Found {(discoveredInteractions() as Array<any>).length} games featuring interactions </p>}
          <button
            type="button"
            class="text-white bg-gradient-to-r from-teal-400 via-teal-500 to-teal-600 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-teal-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center my-4"
            onClick={() => exportToJson()}>
            Export to Clippi
          </button>
        </Show>
      </div>
    </div >
  );
}

export default App;
