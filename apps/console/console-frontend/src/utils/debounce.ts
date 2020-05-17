import { Cancelable } from "./cancelable";

export function debounce<T extends Function>(
    func: T,
    wait = 166,
    immediate = false
): T & Cancelable {
    let timer;
    let result;

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const later = (thisArgs: any, ...args: any[]) => {
        timer = null;
        if (args) {
            result = func.apply(thisArgs, args);
        }
    };

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const debounced: any = function (...args: any[]) {
        if (timer) {
            clearTimeout(timer);
        }
        if (immediate) {
            const callNow = !timer;
            timer = setTimeout(later, wait);
            if (callNow) {
                result = func.apply(this, args);
            }
        } else {
            timer = setTimeout(() => later(this, ...args), wait);
        }
        return result;
    };

    debounced.cancel = () => {
        clearTimeout(timer);
        timer = null;
    };

    return debounced;
}
