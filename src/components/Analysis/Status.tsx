import type { Micrograph } from "../../../src-tauri/bindings/Micrograph";
import type { Segment } from "../../../src-tauri/bindings/Segment";

import { useEffect, useState } from "react";
import { ProgressBar } from "@tremor/react";

type StatusProps = {
    segments: Segment[],
    analysedSegments: number,
    micrograph: Micrograph
}

const Status = ({ segments, micrograph, analysedSegments }: StatusProps) => {
    const [currentStep, setCurrentStep] = useState<{ stepNr: number, stepMessage: string }>({ stepNr: 0, stepMessage: "vorbereiten" });

    useEffect(() => {
        if (micrograph.status == "Pending") {
            setCurrentStep({ stepNr: 1, stepMessage: "importiere Mikroskopaufnahme" });
        } else if (micrograph.status == "Imported") {
            setCurrentStep({ stepNr: 2, stepMessage: "segmentiere Mikroskopaufnahme" });
        } else if (micrograph.status == "Segmented" && analysedSegments !== segments.length) {
            setCurrentStep({ stepNr: 3, stepMessage: "analysiere Segmente" });
        } else if (micrograph.status == "Done") {
            setCurrentStep({ stepNr: 4, stepMessage: "fertig" });
        }
    }, [micrograph, segments, analysedSegments])

    if (currentStep.stepNr == 4 || currentStep.stepNr == 0) return <></>;

    return (
        <div className="bg-white shadow sm:rounded-lg">
            <div className="px-4 py-5 sm:px-6 space-y-2">
                {currentStep.stepNr}/3: {currentStep.stepMessage}...
                {currentStep.stepNr == 3 && <ProgressBar percentageValue={(analysedSegments / segments.length) * 100} label={`${analysedSegments}/${segments.length}`} />}
            </div>
        </div>
    );

}

export default Status;