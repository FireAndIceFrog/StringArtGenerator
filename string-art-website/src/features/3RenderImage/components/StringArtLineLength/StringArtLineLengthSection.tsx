import { useEffect, useState } from "react";
import { useSelector } from "react-redux";
import type { StringArtState } from "../../../shared/redux/stringArtSlice";
import { useTranslation } from "react-i18next";
import { generateLineLength } from "../../../shared/services/lineLengthService";

export const StringArtLineLengthSection = () => {
    const { isGenerating, currentPath, settings } = useSelector(
        (state: { stringArt: StringArtState }) => state.stringArt
    );

    const [scheduleAdded, setScheduleAdded] = useState(false);
    const [diameter, setDiameter] = useState(1); //meters
    const [currentLength, setCurrentLength] = useState<number | null>(null);
    
    const i18next = useTranslation();

    useEffect(() => {
        if(isGenerating) {
            setCurrentLength(null);
            setScheduleAdded(true);
            return;
        }
        if (scheduleAdded && !isGenerating && currentPath.length > 0) {
            setCurrentLength(null);
            generateLineLength({
                diameter_m: diameter,
                num_nails: settings.num_nails,
                path: new Uint32Array(currentPath),
                slack_pct: 0.05,
            }).then(setCurrentLength)
        }
    }, [isGenerating, currentPath, diameter, settings, scheduleAdded]);



    return (
        <section>
            <h2>{i18next.t("Line length title")}</h2>
            <div>
                <label htmlFor="line-length">{i18next.t("Line length diameter Label")}{diameter}</label>
                <input
                    type="range"
                    id="line-length"
                    name="line-length"
                    min={0.1}
                    max={2}
                    step={0.1}
                    value={diameter}
                    onChange={(e) => setDiameter(Number(e.target.value))}
                />
            </div>

            <div>
                {currentLength !== null ? (
                    <span>
                        {i18next.t("Line length output label") + currentLength.toFixed(2) + " m"}
                    </span>
                ) : (
                    <span>{i18next.t("Line length calculating label")}</span>
                )}
            </div>
        </section>
    )

}