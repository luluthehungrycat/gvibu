"""false command - always returns unsuccessful exit status."""


def run(args: list) -> int:
    """Execute false command.
    
    Args:
        args: Command-line arguments (ignored)
        
    Returns:
        Exit code 1 (failure)
    """
    return 1
