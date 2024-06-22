def encode(input: str) -> str:
    """
    Encodes a json string to lightroom s struct

    Args:
        input: json string

    Returns:
        lightroom s struct

    example:
    >>> encode('{"Exposure": 1.5, "Contrast": 30}')
    '"s = { Exposure = 1.5,\nContrast = 30}'
    """


def decode(input: str) -> str:
    """
    Decodes a lightroom s struct to json string

    Args:
        input: lightroom s struct

    Returns:
        json string

    example:
    >>> decode('"s = { Exposure = 1.5,\nContrast = 30}"')
    {"Exposure": 1.5, "Contrast": 30}
    """
