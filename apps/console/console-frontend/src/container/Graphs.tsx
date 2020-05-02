import * as React from 'react';
import { GraphList, GraphListItem } from "../components/GraphList";

const items: { [key: string]: GraphListItem } = {
    "aaaa": { name: "aaaa", labels:["one", "a1", "two"], isActive: false },
    "bbbb": { name: "bbbb", labels:["b1", "b1v"], isActive: false },
};

export const Graphs = () => {
    const [state, setState] = React.useState(items);
    const onChangeActive = React.useCallback((name: string, isActive: boolean) => {
        setState(prev => {
            console.log(`onchange ${name}, ${isActive}`);
            return {
                ...prev,
                [name]: {
                    ...prev[name],
                    isActive
                }
            };
        });
    }, []);

    return <GraphList items={state} onChangeActive={onChangeActive} />;
};
