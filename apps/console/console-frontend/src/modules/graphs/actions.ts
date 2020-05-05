
export interface ToggleActive {
    type: "ToggleActive",
    payload: {
        name: string,
        isActive: boolean,
    }
}

export interface List {
    type: "List",
}

export type Actions = ToggleActive | List;
