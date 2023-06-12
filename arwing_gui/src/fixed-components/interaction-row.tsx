import { Component, Accessor, createSignal } from "solid-js";

import { createOptions } from "@thisbeyond/solid-select";

import { Select } from "../common-components/select";
import { InternalCharacters, characters } from "../consts/characters";

type InteractionProps = {
  playerId: Accessor<number>,
  opponentId: Accessor<number>,
  setInteraction: (characterId: number, moveId: number, withinFrames: number) => void,
}

const [fromId, setFromId] = createSignal(0);
const [move, setMove] = createSignal("");

const InteractionRow: Component<InteractionProps> = (props) => {

  return (
    <div class="w-full grid grid-cols-3">
      <Select
        placeholder="Character"
        onChange={(e) =>
          setFromId(parseInt(InternalCharacters[e]))
        }
        {...createOptions([characters[props.playerId()].name, characters[props.opponentId()].name])}
      />
      <Select
        placeholder="Move"
        onChange={(e) =>
          //Todo: refactor moves, obviously the way it's currently setup isn't going to work
          setMove(e)
        }
        //TODO(Tweet): Refactor Interactions and interactionRows to have state on each row for selected character
        {...createOptions(characters[fromId()].moves.map(move => move.moveName))}
      />
      <div class="col-span-1 relative">
        <input type="text" id="floating_outlined" class="block py-3 px-2 w-full border border-gray-200 rounded leading-normal focus:outline-none focus:ring-4 focus:border-blue-600 peer" placeholder=" " />
        <label for="floating_outlined" class="absolute text-sm text-gray-500 duration-300 transform -translate-y-4 scale-75 top-2 z-10 origin-[0] bg-white px-2 peer-focus:px-2 peer-focus:text-blue-600 peer-placeholder-shown:scale-100 peer-placeholder-shown:-translate-y-1/2 peer-placeholder-shown:top-1/2 peer-focus:top-2 peer-focus:scale-75 peer-focus:-translate-y-4 left-1">Frames Until This Move</label>
      </div>
    </div>
  );
}

export default InteractionRow;
