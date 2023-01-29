import { Card, Text, Metric } from "@tremor/react";
import type { Micrograph } from "../../../src-tauri/bindings/Micrograph";
import type { Segment } from "../../../src-tauri/bindings/Segment";

type AnalysisProps = {
    micrograph: Micrograph
    segments?: Segment[]
}

const Analysis = ({ segments }: AnalysisProps) => {
    return (
        <div className="grid grid-cols-2 gap-4">
            <Card>
                <Text>gefundene Segmente</Text>
                <Metric>{segments?.length}</Metric>
            </Card>

            <Card>
                <Text>durchschnittliche Länge</Text>
                <Metric>{getAvgSegmentLength(segments)}</Metric>
            </Card>

            <Card>
                <Text>durchschnittliche Breite</Text>
                <Metric>{getAvgSegmentWidth(segments)}</Metric>
            </Card>
        </div>
    );
}

const getAvgSegmentLength = (segments?: Segment[]) => {
    let sum = 0;

    if (!segments) return 0;

    segments.forEach(segment => {
        sum += segment.measured_length!;
    });
    return (sum / segments.length).toFixed(2);
}

const getAvgSegmentWidth = (segments?: Segment[]) => {
    let sum = 0;

    if (!segments) return 0;

    segments.forEach(segment => {
        sum += segment.measured_width!;
    });
    return (sum / segments.length).toFixed(2);
}

export default Analysis