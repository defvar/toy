import * as React from "react";
import Chip from "@mui/material/Chip";
import Paper from "@mui/material/Paper";
import { styled } from "@mui/material/styles";

export interface LabelsProps {
    labels: { key: string; display: string }[];
}

const StyledPaper = styled(Paper)(({ theme }) => ({
    display: "flex",
    justifyContent: "left",
    flexWrap: "wrap",
    listStyle: "none",
    padding: theme.spacing(0.5),
    margin: 0,
    backgroundColor: "inherit",
}));

const StyledChip = styled(Chip)(({ theme }) => ({
    margin: theme.spacing(0.5),
}));

export const LabelChips = (props: LabelsProps) => {
    return (
        <StyledPaper elevation={0}>
            {props.labels.map((data) => {
                return (
                    <li key={data.key}>
                        <StyledChip label={data.display} />
                    </li>
                );
            })}
        </StyledPaper>
    );
};
