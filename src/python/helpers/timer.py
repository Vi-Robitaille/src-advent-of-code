import time

def time_func(func):
    """
    This is a simple decorator to make timing stuff easier
    Maybe dont do the sys path shenanigans if you're a real gamer
    Maybe import this in a cleaner way
        You're not my dad tho so i'm going to do it like this
    
    Usage:
    
    # Import helper functions
    helper_location = Path(src, "..", "..", "timer")
    sys.path.insert(1, helper_location.as_posix())
    from timer import time_func

    @timer.time_func
    def f(x):
        print("something")
        x += 1
    
    """
    def timer_wrapper(*args, **kwargs):
        start = time.time()
        result = func(*args, **kwargs)
        print(f"Exec of: {func.__name__}\n\tTook {(time.time() - start):.4f} seconds")
        return result
    return timer_wrapper

