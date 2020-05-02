import * as React from 'react';
import { YamlEditor, YamlEditorProps } from "../components/YamlEditor";
import { useParams } from "react-router-dom";

export const GraphEdit = () => {
    const { name } = useParams();
    return (
        <div>
            <p>{name}</p>
            <YamlEditor />
        </div>
    );
};
