import React from 'react';

interface GenerateInsightsButtonProps {
  onClick: () => void;
}

const GenerateInsightsButton: React.FC<GenerateInsightsButtonProps> = ({ onClick }) => (
  <button
    className="bg-blue-500 hover:bg-blue-700 text-white text-sm xs:text-base font-bold mt-2 -mb-2 py-2 px-4 rounded focus:outline-none focus:shadow-outline"
    type="button"
    onClick={ onClick }
  >
    Generate Insights
  </button>
);

export default GenerateInsightsButton;
