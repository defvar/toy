export interface Rule {
    resources: Array<string>;
    verbs: Array<string>;
}

export interface Role {
    name: string;
    note?: string;
    rules: Array<Rule>;
}

export interface RoleList {
    readonly count: number;
    readonly items: Role[];
}

export interface Subject {
    kind: "User" | "ServiceAccount";
    name: String;
}

export interface RoleBinding {
    name: String;
    role: String;
    subjects: Subject[];
}

export interface RoleBindingList {
    readonly items: RoleBinding[];
    readonly count: number;
}
