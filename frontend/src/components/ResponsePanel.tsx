import React from 'react';

import { ExclamationCircleIcon, ExclamationTriangleIcon, CheckCircleIcon } from '@heroicons/react/24/outline';

interface ResponsePanelProps {
    time: number;
    explanationMessage: [string, number];
}

const iconStyles: string[] = [
    'text-red-500', // Error
    'text-yellow-500', // Warning
    'text-green-500', // Success
];

const icons = [
    <ExclamationCircleIcon className="h-5 w-5" />, // Error icon
    <ExclamationTriangleIcon className="h-5 w-5" />, // Warning icon
    <CheckCircleIcon className="h-5 w-5" />, // Success icon
];

const ResponsePanel: React.FC<ResponsePanelProps> = ({ time, explanationMessage }) => {
    const [message, assessment] = explanationMessage;
    const icon = icons[assessment];
    const iconStyle = iconStyles[assessment];

    return (
        <div className="flex items-center space-x-2 p-4 rounded-lg bg-gray-800 text-white w-full">
          <div className={`flex-shrink-0 ${iconStyle}`}>
            {icon}
          </div>
          <div className="flex-grow">
            <p>{message}</p>
            <p>{`Time: ${time}s`}</p>
          </div>
        </div>
      );
};

export default ResponsePanel;
