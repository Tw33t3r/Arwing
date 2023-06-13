import { createOptions } from "@thisbeyond/solid-select";
import { Component, Accessor, createSignal } from "solid-js";

import { Select } from "../common-components/select";
import { InternalCharacters, characters, Character } from "../consts/characters";

type InteractionProps = {
  player: Accessor<Character>,
  opponent: Accessor<Character>,
  interactionId: number,
  setCharacterId: (characterId: number, interactionId: number) => void,
  setMoveId: (moveId: number, interactionId: number) => void,
  setWithinFrames: (WithinFrames: number, interactionId: number) => void,
}

const [from, setFrom] = createSignal(characters[0]);
const InteractionRow: Component<InteractionProps> = (props) => {

  return (
    <div class="w-full grid grid-cols-3">
      <Select
        placeholder="Character"
        onChange={(e) => {
          props.setCharacterId(e.internalId, props.interactionId);
          setFrom(e);
        }}
        {...createOptions([props.player(), props.opponent()], { key: "name" })}
      />
      <Select
        placeholder="Move"
        onChange={(e) => {
          props.setMoveId(e.moveId as any as number, props.interactionId);
        }}
        {...createOptions(from().moves, { key: "moveName" })}
      />
      <div class="col-span-1 relative">
        <input
          type="text"
          id="floating_outlined"
          class="block py-3 px-2 w-full border border-gray-200 rounded leading-normal focus:outline-none focus:ring-4 focus:border-blue-600 peer"
          onChange={(e) => {
            //TODO(Tweet): Validate user input here
            props.setWithinFrames(parseInt(e.target.value), props.interactionId);
          }}
        />
        <label for="floating_outlined" class="absolute text-sm text-gray-500 duration-300 transform -translate-y-4 scale-75 top-2 z-10 origin-[0] bg-white px-2 peer-focus:px-2 peer-focus:text-blue-600 peer-placeholder-shown:scale-100 peer-placeholder-shown:-translate-y-1/2 peer-placeholder-shown:top-1/2 peer-focus:top-2 peer-focus:scale-75 peer-focus:-translate-y-4 left-1">Frames Until This Move</label>
      </div>
    </div>
  );
}

export default InteractionRow;
