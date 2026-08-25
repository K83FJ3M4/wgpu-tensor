struct FastDivU32 {
    divisor: u32,
    magic: u32,
    shift: u32,
    data: u32
}

struct BroadcastInfo {
    divisions: array<FastDivU32, 7>,
}

fn flattenTensorIndex(info: BroadcastInfo, tensorIndex: array<u32, 8>) -> vec2u {
    let maskData = info.divisions[1].data;
    let numDimensions = info.divisions[0].data;
    let masks = vec2u(maskData & 0xffffu, maskData >> 16u);

    let maxIndex = max(numDimensions, 1u) - 1u;
    let highestValid = vec2<bool>((masks >> vec2u(maxIndex)) & vec2u(1u));
    var flat = select(vec2u(0u), vec2u(tensorIndex[maxIndex]), highestValid);

    for (var i = maxIndex; i != 0;) {
        i -= 1;
        let valid = vec2<bool>((masks >> vec2u(i)) & vec2u(1u));
        let size = select(vec2u(1u), vec2u(info.divisions[i].divisor), valid);
        let item = select(vec2u(0u), vec2u(tensorIndex[i]), valid);
        flat = flat * size + item;
    }

    return flat;
}

fn tensorIndex(info: BroadcastInfo, flatIndex: u32) -> array<u32, 8> {
    let numDimensions = info.divisions[0].data;
    let maxDimension = max(numDimensions, 1u) - 1u;
    var index = flatIndex;
    var tensorIndex: array<u32, 8>;

    for (var i = 0u; i < maxDimension; i += 1u) {
        let division = info.divisions[i];
        let quotient = fastDiv(division, index);
        tensorIndex[i] = index - quotient * division.divisor;
        index = quotient;
    }

    tensorIndex[maxDimension] = index;
    return tensorIndex;
}

fn fastDiv(division: FastDivU32, numerator: u32) -> u32 {
    if (division.divisor == 1u) { return numerator; }

    let quotient = mulHiU32(numerator, division.magic);
    let intermediate = ((numerator - quotient) >> 1u) + quotient;
    return intermediate >> division.shift;
}

fn mulHiU32(lhs: u32, rhs: u32) -> u32 {
    let lhsLow = lhs & 0xffffu;
    let lhsHigh = lhs >> 16u;
    let rhsLow = rhs & 0xffffu;
    let rhsHigh = rhs >> 16u;
    let lowProduct = lhsLow * rhsLow;
    let first = lhsHigh * rhsLow + (lowProduct >> 16u);
    let firstLow = first & 0xffffu;
    let firstHigh = first >> 16u;
    let second = lhsLow * rhsHigh + firstLow;

    return lhsHigh * rhsHigh + firstHigh + (second >> 16u);
}