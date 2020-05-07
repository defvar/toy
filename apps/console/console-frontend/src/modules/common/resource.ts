export interface Resource<T> {
    read: () => T
}

export const toResource = <T>(fn: () => Promise<T>): Resource<T> => {
    let status = 'pending';
    let result;

    const suspender = fn().then(
        (r) => {
            status = 'fulfilled';
            result = r;
        },
        (e) => {
            status = 'rejected';
            result = e;
        });

    const read = () => {
        if (status === 'pending') {
            throw suspender;
        } else if (status === 'rejected') {
            throw result;
        } else {
            return result;
        }
    };

    return { read };
}
