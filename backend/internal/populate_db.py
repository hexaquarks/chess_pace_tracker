import requests
import time, datetime
from dataclasses import dataclass, field
from typing import List, Tuple, Dict, Any, Set
from enum import Enum

BIND_ADDRESS: str = "http://localhost:8000/fetch-chess-data"
N_USERS_TO_PROCESS_FOR_GAME_MODE: int = 20

class GameMode(Enum):
    BLITZ = 'blitz'
    RAPID = 'rapid'
    BULLET = 'bullet'

class UserRatingPair:
    def __init__(self, user_name: str, elo_rating: int):
        self.user_name: str = user_name
        self.elo_rating: int = elo_rating
    
    def __iter__(self):
        yield self.user_name
        yield self.elo_rating

@dataclass
class ELOBraketsCache:
    braket_info_impl: Dict[str, int] = field(default_factory=lambda: {
        "0-400": 0,
        "400-800": 0,
        "800-1200": 0,
        "1200-1600": 0,
        "1600-2000": 0,
        "2000-2400": 0,
    })
    
    def add_data(self, elo_rating: int):
        match elo_rating:
            case _ if 0 <= elo_rating < 400:
                self.braket_info_impl["0-400"] += 1
            case _ if 400 <= elo_rating < 800:
                self.braket_info_impl["400-800"] += 1
            case _ if 800 <= elo_rating < 1200:
                self.braket_info_impl["800-1200"] += 1
            case _ if 1200 <= elo_rating < 1600:
                self.braket_info_impl["1200-1600"] += 1
            case _ if 1600 <= elo_rating < 2000:
                self.braket_info_impl["1600-2000"] += 1
            case _ if 2000 <= elo_rating < 2400:
                self.braket_info_impl["2000-2400"] += 1
            case _:
                print(f"ELO rating {elo_rating} is out of defined brackets.")
    
    def get_next_user_rating_pair(self, user_rating_pairs: List[UserRatingPair]) -> UserRatingPair:
        sorted_brackets = sorted(self.braket_info_impl.items(), key=lambda x: x[1])
        
        bracket_ranges = {
            "0-400": (0, 400),
            "400-800": (400, 800),
            "800-1200": (800, 1200),
            "1200-1600": (1200, 1600),
            "1600-2000": (1600, 2000),
            "2000-2400": (2000, 2400),
        }

        for bracket, _ in sorted_brackets:
            lower_bound, upper_bound = bracket_ranges[bracket]
            for i, user_info in enumerate(user_rating_pairs):
                if lower_bound <= user_info.elo_rating < upper_bound:
                    # We remove the user from the list since we don't want to process twice
                    return user_rating_pairs.pop(i)
                
        # Default on the last user in the list
        return user_rating_pairs.pop()

@dataclass
class BackendRequestPayload:
    user_name: str
    games_count: int
    game_mode: GameMode
    user_color: str
    user_elo: int
    
    def to_json(self) -> Dict[str, Any]:
        return {
            "username": self.user_name,
            "games_count": self.games_count,
            "game_mode": self.game_mode.value,
            "user_color": self.user_color,
            "user_elo": self.user_elo,
        }
    
    @staticmethod
    def from_json(data: Dict[str, Any]) -> "BackendRequestPayload":
        return BackendRequestPayload(
            user_name = data["username"],
            games_count = data["games_count"],
            game_mode = GameMode(data["game_mode"]),
            user_color = data["user_color"],
            user_elo = data["user_elo"]
        )

@dataclass
class BackendResponse:
    average_time: float
    opponents_and_their_rating: List[UserRatingPair]
    
    @staticmethod
    def from_json(data: Dict[str, Any]) -> "BackendResponse":
        return BackendResponse(
            average_time=data["time"],
            opponents_and_their_rating=[
                UserRatingPair(opponent[0], opponent[1]) for opponent in data["players_considered"]
            ]
        )

class Processor:
    def __init__(self):
        self.results: List[List[BackendResponse]] = []
    
    @staticmethod
    def get_starting_user_rating_pairs() -> List[UserRatingPair]:
        return [
            UserRatingPair("Hexaquarks1", 1900),
            UserRatingPair("fifthart", 1800)
        ]
    
    class UserProcessor:
        def __init__(self, game_mode: GameMode):
            self.game_mode: GameMode = game_mode
            self.local_results: List[BackendResponse] = []
            self.elo_braket_info: ELOBraketsCache = ELOBraketsCache()
            
            # Set cache for usernames, to not process same user again
            self.processed_users: Set[str] = set()
            
            # Variables for statistical purposes
            self.nb_processed_users_for_curr_game_mode: int = 0
            self.nb_total_considered_users_for_curr_game_mode: int = 0
            
        def process_user(
            self,
            user_name: str,
            games_count: int,
            game_mode: GameMode, 
            user_color: str,
            user_elo: int) -> BackendResponse:
            
            payload = BackendRequestPayload(
                user_name = user_name,
                games_count = games_count,
                game_mode = game_mode,
                user_color = user_color,
                user_elo = user_elo 
            )
            try:
                response = requests.post(
                    BIND_ADDRESS, 
                    headers={
                        "Content-Type": 'application/json',
                        "x-requested-by": "internal"
                    },
                    json=payload.to_json())
                #response.raise_for_status()
                response_data = response.json()
                
                return BackendResponse.from_json(response_data)
            except Exception as e:
                print(f"A server error occurred while trying to process user {user_name}: {e}")
                raise
            
        def get_next_user_rating_pair(self, user_rating_pairs: List[UserRatingPair]) -> UserRatingPair:
            return self.elo_braket_info.get_next_user_rating_pair(user_rating_pairs)
            
        def process_users_for_game_mode(self):
            # Reset timer
            start_time = time.time()
            
            # Reinitialize the root usernames
            user_rating_pairs: List[UserRatingPair] = Processor.get_starting_user_rating_pairs()
            
            while len(user_rating_pairs) != 0:
                if self.nb_processed_users_for_curr_game_mode > N_USERS_TO_PROCESS_FOR_GAME_MODE:
                    break
                
                self.nb_total_considered_users_for_curr_game_mode += 1
                
                user_info: UserRatingPair = self.get_next_user_rating_pair(user_rating_pairs)
                user_name = user_info.user_name
                user_elo = user_info.elo_rating     
                
                if user_name in self.processed_users:
                    continue 
                    
                print(f"[{datetime.datetime.now()}] Game mode {self.game_mode.value} - processing user: {user_name}")
            
                try: 
                    result: BackendResponse = self.process_user(user_name, 10, self.game_mode, "both", user_elo)
                    self.local_results.append(result)
                    self.nb_processed_users_for_curr_game_mode += 1
                    
                    # Update ELO brackets
                    self.elo_braket_info.add_data(user_info.elo_rating)
                    
                    user_rating_pairs.extend(result.opponents_and_their_rating)
                    print(f"[{datetime.datetime.now()}] Game mode {self.game_mode.value} - Successfully processed user: {user_name}")
                    
                except Exception as e:
                    print(f"[{datetime.datetime.now()}] Game mode {self.game_mode.value} - An error occurred while trying to process player {user_name}. It was skipped in the analysis")
                
                self.processed_users.add(user_name)
                
            end_time = time.time()
            total_time = end_time - start_time
            
            print(f"Total execution time for game mode: {self.game_mode.value} is: {total_time:.2f} seconds")
            print(f"===== Finished running game mode: {self.game_mode.value} =====")
            
        def print_table(self):
            print(f"\n=== GAME MODE: {self.game_mode.value} ===")
            processed_percentage = (self.nb_processed_users_for_curr_game_mode / self.nb_total_considered_users_for_curr_game_mode) * 100 if self.nb_total_considered_users_for_curr_game_mode > 0 else 0
            print(f"Number of users processed: {self.nb_processed_users_for_curr_game_mode} out of {self.nb_total_considered_users_for_curr_game_mode} ({processed_percentage:.2f}%)\n")
            
            headers = ["[0-400]", "[400-800]", "[800-1200]", "[1200-1600]", "[1600-2000]", "[2000-2400]"]
            values = [
                self.elo_braket_info.braket_info_impl["0-400"],
                self.elo_braket_info.braket_info_impl["400-800"],
                self.elo_braket_info.braket_info_impl["800-1200"],
                self.elo_braket_info.braket_info_impl["1200-1600"],
                self.elo_braket_info.braket_info_impl["1600-2000"],
                self.elo_braket_info.braket_info_impl["2000-2400"]
            ]
            
            # Print the headers
            print(" | ".join(f"{header:^12}" for header in headers))
            print("-" * (13 * len(headers) - 1))
            
            # Print the values
            print(" | ".join(f"{value:^12}" for value in values))


    def process_users_for_all_game_modes(self):
        print(f"=====  Starting script  =====")
        for game_mode in GameMode:
            userProcessor = self.UserProcessor(game_mode)
            userProcessor.process_users_for_game_mode()
            userProcessor.print_table()
            self.results.append(userProcessor.local_results)
            
        print(f"=====  Script finished running. Results:  =====")

if __name__ == "__main__":
    processor = Processor()
    processor.process_users_for_all_game_modes()
