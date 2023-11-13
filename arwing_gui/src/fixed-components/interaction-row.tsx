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
    <div class="lg:w-2/3 grid grid-cols-3">
      <Select
        placeholder="Character"
        class="px-2"
        onChange={(e) => {
          props.setCharacterId(e.internalId, props.interactionId);
          setFrom(e);
        }}
        {...createOptions([props.player(), props.opponent()], { key: "name" })}
      />
      <Select
        placeholder="Move"
        class="px-2"
        onChange={(e) => {
          props.setMoveId(e.moveId as any as number, props.interactionId);
        }}
        {...createOptions(from().moves, { key: "moveName" })}
      />
      <div class="col-span-1 relative px-2">
        <div class="py-1 px-2 mr-2 border border-gray-200 rounded ">
          <input
            type="text"
            id="floating_outlined"
            placeholder=" "
            onChange={(e) => {
              //TODO(Tweet): Validate user input here
              props.setWithinFrames(parseInt(e.target.value), props.interactionId);
            }}
            class="p-4 pl-10 w-full text-sm border border-gray-300 rounded-lg bg-gray-50 focus:outline-none focus:ring-2 focus:border-blue-600 peer"
          />
          <label
            for="floating_outlined"
            class="absolute text-sm text-gray-400 bg-gray-50 left-10 duration-300 transform -translate-y-4 scale-75 top-2 z-10 origin-[0] px-2 peer-focus:px-2 peer-focus:text-blue-600 peer-placeholder-shown:scale-100 peer-placeholder-shown:-translate-y-1/2 peer-placeholder-shown:top-1/2 peer-focus:top-2 peer-focus:scale-75 peer-focus:-translate-y-4"
          >
            Frames Until This Move
          </label>
        </div>
      </div>
    </div>
  );
}

export default InteractionRow;
