export interface JsonSchema {
    $id?: string;
    $ref?: string;
    $schema?: string;

    type?: string;
    enum?: string[];

    const?: string;
    oneOf?: JsonSchema[];

    required?: string[];
    properties?: { [key: string]: JsonSchema };

    definitions?: { [key: string]: JsonSchema };

    title?: string;
    description?: string;
}

export type Result<T, E> = Success<T> | Failure<E>;

export class Success<T> {
    constructor(readonly value: T) {}

    isSuccess(): this is Success<T> {
        return true;
    }

    isFailure(): this is Failure<unknown> {
        return false;
    }
}

export class Failure<E> {
    constructor(readonly error: E) {}

    isSuccess(): this is Success<unknown> {
        return false;
    }

    isFailure(): this is Failure<E> {
        return true;
    }
}
