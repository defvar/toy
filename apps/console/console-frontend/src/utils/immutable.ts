/* eslint-disable @typescript-eslint/no-explicit-any */
import produce, { Immutable } from "immer";

type Tail<T extends any[]> = ((...t: T) => any) extends (
    _: any,
    ...tail: infer TT
) => any
    ? TT
    : [];

type State<T> = Immutable<T>;

export const nextState = <
    Recipe extends (...args: any[]) => any,
    Params extends any[] = Parameters<Recipe>,
    T = Params[0]
>(
    f: Recipe,
    initial: State<T>
): (<Base extends State<T>>(base?: Base, ...rest: Tail<Params>) => T) => {
    return produce(f, initial);
};
