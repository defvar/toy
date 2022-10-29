import * as React from "react";
import { GraphList } from "./GraphList";
import {
    reducer,
    GraphListState,
    initialState,
    GraphListItemState,
    Actions,
} from "../../modules/graphs";
import { toResource, Resource } from "../../modules/common";
import CircularProgress from "@mui/material/CircularProgress";
import RefreshIcon from "@mui/icons-material/Refresh";
import IconButton from "@mui/material/IconButton";

const items: { [key: string]: GraphListItemState } = {
    "example-tick": {
        name: "example-tick",
        labels: ["one", "a1", "two"],
        isActive: false,
        isLoading: false,
    },
    bbbb: {
        name: "bbbb",
        labels: ["b1", "b1v"],
        isActive: false,
        isLoading: false,
    },
};

const items2: { [key: string]: GraphListItemState } = {
    cccc: {
        name: "cccc",
        labels: ["1", "2", "3"],
        isActive: false,
        isLoading: false,
    },
    dddd: {
        name: "dddd",
        labels: ["21", "22"],
        isActive: false,
        isLoading: false,
    },
};

const fetch = (id: number) => {
    const promise = new Promise<{ [key: string]: GraphListItemState }>(
        (resolve) => {
            setTimeout(() => {
                if (id === 0) {
                    resolve(items);
                } else {
                    resolve(items2);
                }
            }, 1000);
        }
    );
    return toResource(async () => await promise);
};

interface GraphListSuspenseProps {
    state: GraphListState;
    dispatch: React.Dispatch<Actions>;
    resource: Resource<{ [key: string]: GraphListItemState }>;
}

const GraphListSuspense = (props: GraphListSuspenseProps) => {
    const d = props.resource.read();
    React.useEffect(() => {
        props.dispatch({ type: "List", payload: { items: d } });
    }, [d]);
    return <GraphList items={props.state.items} dispatch={props.dispatch} />;
};

export const Graphs = () => {
    const [resource, setResource] = React.useState(() => fetch(0));
    const [state, dispatch] = React.useReducer(reducer, initialState);
    const onClick = React.useCallback(() => {
        setResource(() => fetch(1));
    }, []);
    return (
        <>
            <IconButton aria-label="refresh" onClick={onClick} size="large">
                <RefreshIcon />
            </IconButton>
            <React.Suspense
                fallback={
                    <div
                        style={{
                            display: "flex",
                            alignItems: "center",
                            justifyContent: "center",
                        }}
                    >
                        <CircularProgress
                            size={68}
                            sx={{
                                progress: {
                                    position: "absolute",
                                    top: "50%",
                                    left: "50%",
                                    marginTop: -12,
                                    marginLeft: -12,
                                },
                            }}
                        />
                    </div>
                }
            >
                <GraphListSuspense
                    state={state}
                    dispatch={dispatch}
                    resource={resource}
                />
            </React.Suspense>
        </>
    );
};

export default Graphs;
