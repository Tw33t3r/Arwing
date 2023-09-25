import {
  Show,
  For,
  splitProps,
  mergeProps,
  Component,
  ParentComponent,
  createEffect,
  on,
  createContext,
  useContext,
} from "solid-js";

import {
  createSelect,
} from "@thisbeyond/solid-select";

//TODO(Tweet): Trim down code duplication here
type OptionType = any;

type SingleValue = any;

type ValueType = SingleValue | SingleValue[];

interface CreateSelectProps {
  options: OptionType[] | ((inputValue: string) => OptionType[]);
  initialValue?: ValueType;
  multiple?: boolean;
  disabled?: boolean;
  optionToValue?: (option: OptionType) => SingleValue;
  isOptionDisabled?: (option: OptionType) => boolean;
  onChange?: (value: ValueType) => void;
  onInput?: (inputValue: string) => void;
}

interface CommonProps {
  format: (
    data: OptionType | ValueType,
    type: "option" | "value"
  ) => string | undefined;
  placeholder?: string;
  id?: string;
  name?: string;
  class?: string;
  autofocus?: boolean;
  readonly?: boolean;
  loading?: boolean;
  loadingPlaceholder?: string;
  emptyPlaceholder?: string;
}

type SelectReturn = ReturnType<typeof createSelect>;

type SelectProps = CreateSelectProps & Partial<CommonProps>;

const SelectContext = createContext<SelectReturn>();

const useSelect = () => {
  const context = useContext(SelectContext);
  if (!context) throw new Error("No SelectContext found in ancestry.");
  return context;
};

const Select: Component<SelectProps> = (props) => {
  const [selectProps, local] = splitProps(
    mergeProps(
      {
        format: ((data, type) => data) as CommonProps["format"],
        placeholder: "Select...",
        readonly: typeof props.options !== "function",
        loading: false,
        loadingPlaceholder: "Loading...",
        emptyPlaceholder: "No options",
      },
      props
    ),
    [
      "options",
      "optionToValue",
      "isOptionDisabled",
      "multiple",
      "disabled",
      "onInput",
      "onChange",
    ]
  );
  const select = createSelect(selectProps);

  createEffect(
    on(
      () => local.initialValue,
      (value) => value !== undefined && select.setValue(value)
    )
  );

  return (
    <SelectContext.Provider value={select}>
      <Container class={local.class}>
        <Control
          id={local.id}
          name={local.name}
          format={local.format}
          placeholder={local.placeholder}
          autofocus={local.autofocus}
          readonly={local.readonly}
        />
        <List
          loading={local.loading}
          loadingPlaceholder={local.loadingPlaceholder}
          emptyPlaceholder={local.emptyPlaceholder}
          format={local.format}
        />
      </Container>
    </SelectContext.Provider>
  );
};

type ContainerProps = Pick<CommonProps, "class">;

const Container: ParentComponent<ContainerProps> = (props) => {
  const select = useSelect();
  return (
    <div
      class={`col-span-1 data-disabled:(pointer-events-none) ${props.class !== undefined ? props.class : ""
        }`}
      data-disabled={select.disabled}
      onFocusIn={select.onFocusIn}
      onFocusOut={select.onFocusOut}
      onMouseDown={(event) => {
        select.onMouseDown(event);
        event.currentTarget.getElementsByTagName("input")[0].focus();
      }}
    >
      {props.children}
    </div>
  );
};

type ControlProps = Omit<CommonProps, "class">;

const Control: Component<ControlProps> = (props) => {
  const select = useSelect();

  const removeValue = (index: number) => {
    const value = select.value();
    select.setValue([...value.slice(0, index), ...value.slice(index + 1)]);
  };

  return (
    <div
      class="py-1 px-2 border border-gray-200 rounded leading-normal "
      data-multiple={select.multiple}
      data-has-value={select.hasValue()}
      data-disabled={select.disabled}
      onClick={select.onClick}
    >
      <Show when={!select.hasValue() && !select.hasInputValue()}>
        <Placeholder>{props.placeholder}</Placeholder>
      </Show>
      <Show
        when={select.hasValue() && !select.multiple && !select.hasInputValue()}
      >
        <SingleValue>{props.format(select.value(), "value")}</SingleValue>
      </Show>
      <Show when={select.hasValue() && select.multiple}>
        <For each={select.value()}>
          {(value, index) => (
            <MultiValue onRemove={() => removeValue(index())}>
              {props.format(value, "value")}
            </MultiValue>
          )}
        </For>
      </Show>
      <Input
        id={props.id}
        name={props.name}
        autofocus={props.autofocus}
        readonly={props.readonly}
      />
    </div>
  );
};

type PlaceholderProps = Pick<CommonProps, "placeholder">;

const Placeholder: ParentComponent<PlaceholderProps> = (props) => {
  return <div class="absolute text-sm p-4 pl-10 z-10 text-gray-400">{props.children}</div>;
};

const SingleValue: ParentComponent<{}> = (props) => {
  return <div class="absolute text-sm text-gray-900 p-4 pl-10 z-10">{props.children}</div>;
};

const MultiValue: ParentComponent<{ onRemove: () => void }> = (props) => {
  const select = useSelect();

  return (
    <div class="flex items-center rounded px-[4px] bg-gray-100 text-[85%] leading-[inherit]">
      {props.children}
      <button
        type="button"
        class="solid-select-multi-value-remove"
        onClick={(event: MouseEvent) => {
          event.stopPropagation();
          props.onRemove();
        }}
      >
        ⨯
      </button>
    </div>
  );
};

type InputProps = Pick<CommonProps, "id" | "name" | "autofocus" | "readonly">;

const Input: Component<InputProps> = (props) => {
  const select = useSelect();
  return (
    <input
      id={props.id}
      name={props.name}
      class="p-4 pl-10 w-full text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50
      focus:ring-blue-500 focus:border-blue-500"
      data-multiple={select.multiple}
      data-is-active={select.isActive()}
      type="text"
      tabIndex={0}
      autocomplete="off"
      autocapitalize="none"
      autofocus={props.autofocus}
      readonly={props.readonly}
      disabled={select.disabled}
      size={1}
      value={select.inputValue()}
      onInput={select.onInput}
      onKeyDown={(event: KeyboardEvent) => {
        select.onKeyDown(event);
        if (!event.defaultPrevented) {
          if (event.key === "Escape") {
            event.preventDefault();
            event.stopPropagation();
            (event.target as HTMLElement).blur();
          }
        }
      }}
      onMouseDown={(event) => {
        event.stopPropagation();
      }}
    />
  );
};

type ListProps = Pick<
  CommonProps,
  "loading" | "loadingPlaceholder" | "emptyPlaceholder" | "format"
>;

const List: Component<ListProps> = (props) => {
  const select = useSelect();

  return (
    <Show when={select.isOpen()}>
      <div class="z-20 bg-gray-50 absolute min-w-fit shadow-lg whitespace-nowrap rounded-sm mt-1 p-2 z-1 overflow-y-auto max-h-[50vh]" >
        <Show
          when={!props.loading}
          fallback={
            <div class="px-4 py-2 cursor-default select-none">
              {props.loadingPlaceholder}
            </div>
          }
        >
          <For
            each={select.options()}
            fallback={
              <div class="px-4 py-2 cursor-default select-none">
                {props.emptyPlaceholder}
              </div>
            }
          >
            {(option: OptionType) => (
              <Option option={option}>{props.format(option, "option")}</Option>
            )}
          </For>
        </Show>
      </div>
    </Show>
  );
};

type OptionProps = {
  option: OptionType;
};

const Option: ParentComponent<OptionProps> = (props) => {
  const select = useSelect();

  const scrollIntoViewOnFocus = (element: HTMLDivElement) => {
    createEffect(() => {
      if (select.isOptionFocused(props.option)) {
        element.scrollIntoView({ block: "nearest" });
      }
    });
  };
  return (
    <div
      ref={scrollIntoViewOnFocus}
      data-disabled={select.isOptionDisabled(props.option)}
      data-focused={select.isOptionFocused(props.option)}
      class="px-4 py-2 cursor-default text-sm text-gray-900 select-none
      hover:bg-gray-200 data-focused:bg-gray-100
      data-disabled:(pointer-events-none text-gray-400)
      mark:(underline text-[unset] bg-[unset])"
      onClick={() => select.pickOption(props.option)}
    >
      {props.children}
    </div>
  );
};

export {
  Select,
};
