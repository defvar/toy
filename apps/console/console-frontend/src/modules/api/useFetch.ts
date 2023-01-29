import * as React from "react";
import { Resource } from "../common/resource";
import { Result } from "../common/types";

let cache = new Map();

export function useFetch<T, E>(
    keys: ReadonlyArray<unknown>,
    fn: () => Resource<Result<T, E>>
): Result<T, E> {
    const ser = JSON.stringify(keys);
    const [state, setState] = React.useState(null);
    React.useEffect(() => {
        setState(() => {
            let r = null;
            if (!cache.has(ser)) {
                console.debug(`crate resource. key:${ser}`);
                r = fn();
                cache.set(ser, r);
            } else {
                r = cache.get(ser);
            }
            return r.read();
        });
    }, [ser]);
    return state;
}
