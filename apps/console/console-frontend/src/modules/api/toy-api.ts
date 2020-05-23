export interface ServiceResponse {
    payload: { id: string }[];
}

export const ToyApi = {
    getServices: (): Promise<ServiceResponse> => {
        return fetch(`http://localhost:3030/services`, {
            method: "GET",
            mode: "cors",
        })
            .then((res) => {
                if (res.ok) {
                    return res.json();
                }
                throw new Error("response was not ok.");
            })
            .then((json) => {
                return {
                    payload: json,
                } as ServiceResponse;
            })
            .catch((error) => {
                console.log(
                    "There has been a problem with your fetch operation: ",
                    error.message
                );
                return {
                    payload: [],
                };
            });
    },
};
