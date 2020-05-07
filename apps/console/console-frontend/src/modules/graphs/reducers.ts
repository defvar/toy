import { GraphListState } from "./types";
import { Actions } from "./actions";

export const initialState: GraphListState = {
    items: {},
};

export const reducer = (state: GraphListState = initialState, action: Actions): GraphListState => {
    switch (action.type) {
        case "List":
            return {
                ...state,
                items: action.payload.items
            };
        case "ToggleActive":
            const { name, isActive } = action.payload;
            return {
                ...state,
                items: {
                    ...state.items,
                    [name]: {
                        ...state.items[name],
                        isActive
                    }
                }
            };
    }
}
