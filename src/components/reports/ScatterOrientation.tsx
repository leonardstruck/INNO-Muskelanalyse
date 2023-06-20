import { ResponsiveContainer, Scatter, ScatterChart, XAxis, YAxis, ZAxis } from "recharts";
import { CachedSegment } from "../../../src-tauri/bindings/CachedSegment"

type ScatterOrientationProps = {
    segments: CachedSegment[]
}

const ScatterOrientation = ({ segments }: ScatterOrientationProps) => {
    const data = segments.map((segment) => ({
        x: segment.location_x,
        y: segment.location_y ? segment.location_y * -1 : 0,
        angle: segment.measured_angle
    }));

    return (
        <ResponsiveContainer width={600} height={600} className="bg-slate-800">
            <ScatterChart>
                <XAxis type="number" dataKey="x" name="x" hide />
                <YAxis type="number" dataKey="y" name="y" hide />
                <ZAxis type="number" dataKey="angle" name="angle" />
                <Scatter name="Orientation" fill="white" data={data} shape={CustomShape} />
            </ScatterChart>
        </ResponsiveContainer>
    )
}

const CustomShape = (props: any) => {
    return (
        <svg
            xmlns="http://www.w3.org/2000/svg"
            xmlSpace="preserve"
            viewBox="0 0 512.04 512.04"
            {...props}
            width={20}
            height={20}
        >
            <path transform={`rotate(${props.angle}, 256, 256)`} d="M508.933 248.353 402.267 141.687c-4.267-4.053-10.987-3.947-15.04.213a10.763 10.763 0 0 0 0 14.827l88.427 88.427H36.4l88.427-88.427c4.053-4.267 3.947-10.987-.213-15.04a10.763 10.763 0 0 0-14.827 0L3.12 248.353a10.623 10.623 0 0 0 0 15.04L109.787 370.06c4.267 4.053 10.987 3.947 15.04-.213a10.763 10.763 0 0 0 0-14.827L36.4 266.593h439.147L387.12 355.02c-4.267 4.053-4.373 10.88-.213 15.04 4.053 4.267 10.88 4.373 15.04.213l.213-.213 106.667-106.667c4.266-4.053 4.266-10.88.106-15.04z" />
        </svg>
    );
}

export default ScatterOrientation