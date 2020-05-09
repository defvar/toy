import { GraphEditState } from "./types";
import { Actions } from "./actions";

export const initialState: GraphEditState = {
    services: {},
};

export const reducer = (state: GraphEditState = initialState, action: Actions): GraphEditState => {
    switch (action.type) {
        case "GetServices":
            return {
                ...state,
                services: action.payload.items,
            };
    }
}
