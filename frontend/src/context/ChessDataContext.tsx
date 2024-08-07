import React, { createContext, useContext, ReactNode } from 'react';
import useFetchChessData from '../hooks/useFetchChessData';
import { InputProps } from '../components/panels/InputsPanel';
import { ResponseInformation } from '../services/apiService';

interface ChessDataContextProps {
    response: ResponseInformation | null;
    isLoading: boolean;
    error: string | null;
    fetchData: (props: InputProps) => void;
    usernameNotFound: boolean;
    fetchProgress: string;
}

const ChessDataContext = createContext<ChessDataContextProps | undefined>(undefined);

export const ChessDataProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
    const { response, isLoading, error, fetchData, usernameNotFound, fetchProgress } = useFetchChessData();

    return (
        <ChessDataContext.Provider value={{ response, isLoading, error, fetchData, usernameNotFound, fetchProgress }}>
            {children}
        </ChessDataContext.Provider>
    );
};

export const useChessData = () => {
    const context = useContext(ChessDataContext);
    if (!context) {
        throw new Error('useChessData must be used within a ChessDataProvider');
    }

    return context;
};