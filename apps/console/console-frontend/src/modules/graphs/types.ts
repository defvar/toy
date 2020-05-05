
export interface GraphState {
    name: string,
    labels: string[],
    isActive: boolean,
}

export interface GraphListState {
   items: { [key: string]: GraphState }
}
