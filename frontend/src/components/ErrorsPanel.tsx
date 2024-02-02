import React from 'react';
import { ExclamationTriangleIcon } from '@heroicons/react/24/outline';

interface ErrorsPanelProps {
    gamesWithError: Array<[number, string]>;
}

const ErrorsPanel: React.FC<ErrorsPanelProps> = ({ gamesWithError }) => {
    console.log(gamesWithError);
    if (!gamesWithError || gamesWithError.length === 0) {
        return (
            <div className="flex flex-col items-center space-x-2 p-4 rounded-lg bg-gray-800 text-white w-full">
                <div className="text-center text-white">No errors were found.</div>
            </div>
        )
    }

    const iconSize = 'h-14 w-14';

    return (
        <div className="w-full h-full max-h-[calc(100vh*0.45)] overflow-y-auto self-begin">
            <div className="flex flex-col items-center rounded-lg bg-gray-800 text-white w-full">
                {gamesWithError.map(([gameNumber, errorMessage], i) => (
                    <div key={i} className="flex flex-col items-center w-full pl-2 bg-white rounded-lg shadow md:flex-row hover:bg-gray-100 dark:bg-gray-800 dark:hover:bg-gray-700">
                        <ExclamationTriangleIcon className={`${iconSize} text-yellow-500`} />
                        <div className="flex flex-col justify-between w-full p-4 leading-normal">
                            <span className="text-xl font-bold tracking-tight text-gray-900 dark:text-white">Game {gameNumber}</span>
                            <p className="text-sm font-normal text-wrap text-gray-700 dark:text-gray-400">{errorMessage}</p>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
};

export default ErrorsPanel;
