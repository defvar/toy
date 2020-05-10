import { GraphListItemState } from "./types";

export interface ToggleActive {
    type: "ToggleActive";
    payload: {
        name: string;
        isActive: boolean;
    };
}

export interface List {
    type: "List";
    payload: {
        items: { [key: string]: GraphListItemState };
    };
}

export type Actions = ToggleActive | List;
