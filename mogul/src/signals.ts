import { createSignal, type Accessor } from "solid-js";
import { makePersisted } from "@solid-primitives/storage";
import {
  type Identifier,
  type Func,
  type Context,
  defaultContext,
  type DisplayStyle,
  type FormedExpr,
  type FormedReducedExpr,
} from "../../ski3/pkg/index";

export const [commandStr, setCommandStr] = createSignal("");

export const [displayStyle, setDisplayStyle] = makePersisted(
  createSignal<DisplayStyle>("EcmaScript"),
  {
    name: "display-style",
    storage: localStorage,
  },
);

export const [context, setContext] = makePersisted(
  createSignal<Context>(defaultContext()),
  {
    name: "context",
    storage: sessionStorage,
  },
);

export const [console, setConsole] = createSignal<ConsoleItem[]>([]);

export interface ConsoleItemUpdate {
  type: "Update";
  func: Func;
}

export interface ConsoleItemDelete {
  type: "Delete";
  identifier: Identifier;
}

export interface ConsoleItemReduce {
  type: "Reduce";
  formed: FormedExpr;
  reduceResults: Accessor<
    {
      readonly step: number;
      readonly formed: FormedReducedExpr;
    }[]
  >;
}

export interface ConsoleItemReduceLast {
  type: "ReduceLast";
  formed: FormedExpr;
  reduceResult: Accessor<{
    readonly step: number;
    readonly formed: FormedReducedExpr;
  } | null>;
}

export interface ConsoleItemReduceHead {
  type: "ReduceHead";
  formed: FormedExpr;
  reduceResults: Accessor<
    {
      readonly step: number;
      readonly formed: FormedReducedExpr;
    }[]
  >;
}

export interface ConsoleItemQueryDefined {
  type: "QueryDefined";
  func: Func;
}

export interface ConsoleItemQueryUndefined {
  type: "QueryUndefined";
  identifier: Identifier;
}

export interface ConsoleItemContext {
  type: "Context";
}

export type ConsoleItem =
  | ConsoleItemUpdate
  | ConsoleItemDelete
  | ConsoleItemReduce
  | ConsoleItemReduceLast
  | ConsoleItemReduceHead
  | ConsoleItemQueryDefined
  | ConsoleItemQueryUndefined
  | ConsoleItemContext;
