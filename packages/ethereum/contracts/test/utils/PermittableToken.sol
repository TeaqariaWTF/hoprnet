// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.6.0 <0.9.0;

import "forge-std/Test.sol";

/**
 * Take the ERC677 implementation from blockscout and provider helper function to etch deployed code
 * https://blockscout.com/xdai/mainnet/address/0xf8D1677c8a0c961938bf2f9aDc3F3CFDA759A9d9/contracts#address-tabs
 */
abstract contract PermittableTokenFixtureTest is Test {
    // to alter the storage
    using stdStorage for StdStorage;
    // address constant PERMITTABLE_TOKEN_ADDRESS = 0xf8D1677c8a0c961938bf2f9aDc3F3CFDA759A9d9;

    bytes constant PERMITTABLE_TOKEN_DEPLOYED_CODE = bytes(
        hex"6080604052600436106101b35763ffffffff60e060020a60003504166305d2035b81146101b857806306fdde03146101e1578063095ea7b31461026b5780630b26cf661461028f57806318160ddd146102b257806323b872dd146102d957806330adf81f14610303578063313ce567146103185780633644e5151461034357806339509351146103585780634000aea01461037c57806340c10f19146103ad57806342966c68146103d157806354fd4d50146103e957806366188463146103fe57806369ffa08a1461042257806370a0823114610449578063715018a61461046a578063726600ce1461047f5780637d64bcb4146104a05780637ecebe00146104b5578063859ba28c146104d65780638da5cb5b146105175780638fcbaf0c1461054857806395d89b4114610586578063a457c2d71461059b578063a9059cbb146105bf578063b753a98c146105e3578063bb35783b14610607578063c6a1dedf14610631578063cd59658314610646578063d505accf1461065b578063d73dd62314610694578063dd62ed3e146106b8578063f2d5d56b146106df578063f2fde38b14610703578063ff9e884d14610724575b600080fd5b3480156101c457600080fd5b506101cd61074b565b604080519115158252519081900360200190f35b3480156101ed57600080fd5b506101f661076c565b6040805160208082528351818301528351919283929083019185019080838360005b83811015610230578181015183820152602001610218565b50505050905090810190601f16801561025d5780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b34801561027757600080fd5b506101cd600160a060020a03600435166024356107fa565b34801561029b57600080fd5b506102b0600160a060020a0360043516610810565b005b3480156102be57600080fd5b506102c761086a565b60408051918252519081900360200190f35b3480156102e557600080fd5b506101cd600160a060020a0360043581169060243516604435610870565b34801561030f57600080fd5b506102c7610a38565b34801561032457600080fd5b5061032d610a5c565b6040805160ff9092168252519081900360200190f35b34801561034f57600080fd5b506102c7610a65565b34801561036457600080fd5b506101cd600160a060020a0360043516602435610a6b565b34801561038857600080fd5b506101cd60048035600160a060020a0316906024803591604435918201910135610aac565b3480156103b957600080fd5b506101cd600160a060020a0360043516602435610bbd565b3480156103dd57600080fd5b506102b0600435610cc8565b3480156103f557600080fd5b506101f6610cd5565b34801561040a57600080fd5b506101cd600160a060020a0360043516602435610d0c565b34801561042e57600080fd5b506102b0600160a060020a0360043581169060243516610de9565b34801561045557600080fd5b506102c7600160a060020a0360043516610e0e565b34801561047657600080fd5b506102b0610e29565b34801561048b57600080fd5b506101cd600160a060020a0360043516610e40565b3480156104ac57600080fd5b506101cd610e54565b3480156104c157600080fd5b506102c7600160a060020a0360043516610e5b565b3480156104e257600080fd5b506104eb610e6d565b6040805167ffffffffffffffff9485168152928416602084015292168183015290519081900360600190f35b34801561052357600080fd5b5061052c610e78565b60408051600160a060020a039092168252519081900360200190f35b34801561055457600080fd5b506102b0600160a060020a0360043581169060243516604435606435608435151560ff60a4351660c43560e435610e87565b34801561059257600080fd5b506101f6610fc5565b3480156105a757600080fd5b506101cd600160a060020a036004351660243561101f565b3480156105cb57600080fd5b506101cd600160a060020a0360043516602435611032565b3480156105ef57600080fd5b506102b0600160a060020a0360043516602435611054565b34801561061357600080fd5b506102b0600160a060020a0360043581169060243516604435611064565b34801561063d57600080fd5b506102c7611075565b34801561065257600080fd5b5061052c611099565b34801561066757600080fd5b506102b0600160a060020a036004358116906024351660443560643560ff6084351660a43560c4356110a8565b3480156106a057600080fd5b506101cd600160a060020a0360043516602435611184565b3480156106c457600080fd5b506102c7600160a060020a036004358116906024351661120b565b3480156106eb57600080fd5b506102b0600160a060020a0360043516602435611236565b34801561070f57600080fd5b506102b0600160a060020a0360043516611241565b34801561073057600080fd5b506102c7600160a060020a0360043581169060243516611261565b60065474010000000000000000000000000000000000000000900460ff1681565b6000805460408051602060026001851615610100026000190190941693909304601f810184900484028201840190925281815292918301828280156107f25780601f106107c7576101008083540402835291602001916107f2565b820191906000526020600020905b8154815290600101906020018083116107d557829003601f168201915b505050505081565b600061080733848461127e565b50600192915050565b600654600160a060020a0316331461082757600080fd5b610830816112c0565b151561083b57600080fd5b6007805473ffffffffffffffffffffffffffffffffffffffff1916600160a060020a0392909216919091179055565b60045490565b600080600160a060020a038516151561088857600080fd5b600160a060020a038416151561089d57600080fd5b600160a060020a0385166000908152600360205260409020546108c6908463ffffffff6112c816565b600160a060020a0380871660009081526003602052604080822093909355908616815220546108fb908463ffffffff6112da16565b600160a060020a038086166000818152600360209081526040918290209490945580518781529051919392891692600080516020611d7283398151915292918290030190a3600160a060020a0385163314610a225761095a853361120b565b905060001981146109c457610975818463ffffffff6112c816565b600160a060020a038616600081815260056020908152604080832033808552908352928190208590558051948552519193600080516020611d92833981519152929081900390910190a3610a22565b600160a060020a0385166000908152600a602090815260408083203384529091529020541580610a175750600160a060020a0385166000908152600a602090815260408083203384529091529020544211155b1515610a2257600080fd5b610a2d8585856112ed565b506001949350505050565b7f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c981565b60025460ff1681565b60085481565b336000818152600560209081526040808320600160a060020a03871684529091528120549091610807918590610aa7908663ffffffff6112da16565b61127e565b600084600160a060020a03811615801590610ad05750600160a060020a0381163014155b1515610adb57600080fd5b610ae58686611324565b1515610af057600080fd5b85600160a060020a031633600160a060020a03167fe19260aff97b920c7df27010903aeb9c8d2be5d310a2c67824cf3f15396e4c16878787604051808481526020018060200182810382528484828181526020019250808284376040519201829003965090945050505050a3610b65866112c0565b15610bb157610ba633878787878080601f01602080910402602001604051908101604052809392919081815260200183838082843750611330945050505050565b1515610bb157600080fd5b50600195945050505050565b600654600090600160a060020a03163314610bd757600080fd5b60065474010000000000000000000000000000000000000000900460ff1615610bff57600080fd5b600454610c12908363ffffffff6112da16565b600455600160a060020a038316600090815260036020526040902054610c3e908363ffffffff6112da16565b600160a060020a038416600081815260036020908152604091829020939093558051858152905191927f0f6798a560793a54c3bcfe86a93cde1e73087d944c0ea20544137d412139688592918290030190a2604080518381529051600160a060020a03851691600091600080516020611d728339815191529181900360200190a350600192915050565b610cd233826114ad565b50565b60408051808201909152600181527f3100000000000000000000000000000000000000000000000000000000000000602082015281565b336000908152600560209081526040808320600160a060020a0386168452909152812054808310610d6057336000908152600560209081526040808320600160a060020a0388168452909152812055610d95565b610d70818463ffffffff6112c816565b336000908152600560209081526040808320600160a060020a03891684529091529020555b336000818152600560209081526040808320600160a060020a038916808552908352928190205481519081529051929392600080516020611d92833981519152929181900390910190a35060019392505050565b600654600160a060020a03163314610e0057600080fd5b610e0a828261159c565b5050565b600160a060020a031660009081526003602052604090205490565b600654600160a060020a031633146101b357600080fd5b600754600160a060020a0390811691161490565b6000806000fd5b60096020526000908152604090205481565b600260056000909192565b600654600160a060020a031681565b600080861580610e975750864211155b1515610ea257600080fd5b604080517fea2aa0a1be11a07ed86d755c93467f4f82362b452371d1ba94d1715123511acb6020820152600160a060020a03808d16828401528b166060820152608081018a905260a0810189905287151560c0808301919091528251808303909101815260e0909101909152610f17906115da565b9150610f25828686866116e1565b600160a060020a038b8116911614610f3c57600080fd5b600160a060020a038a1660009081526009602052604090208054600181019091558814610f6857600080fd5b85610f74576000610f78565b6000195b905085610f86576000610f88565b865b600160a060020a03808c166000908152600a60209081526040808320938e1683529290522055610fb98a8a836118e3565b50505050505050505050565b60018054604080516020600284861615610100026000190190941693909304601f810184900484028201840190925281815292918301828280156107f25780601f106107c7576101008083540402835291602001916107f2565b600061102b8383610d0c565b9392505050565b600061103e8383611324565b151561104957600080fd5b6108073384846112ed565b61105f338383610870565b505050565b61106f838383610870565b50505050565b7fea2aa0a1be11a07ed86d755c93467f4f82362b452371d1ba94d1715123511acb81565b600754600160a060020a031690565b600080428610156110b857600080fd5b600160a060020a03808a1660008181526009602090815260409182902080546001810190915582517f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c99281019290925281830193909352928b166060840152608083018a905260a0830182905260c08084018a90528151808503909101815260e090930190529250611149906115da565b9050611157818686866116e1565b600160a060020a038a811691161461116e57600080fd5b61117989898961127e565b505050505050505050565b336000908152600560209081526040808320600160a060020a03861684529091528120546111b8908363ffffffff6112da16565b336000818152600560209081526040808320600160a060020a038916808552908352928190208590558051948552519193600080516020611d92833981519152929081900390910190a350600192915050565b600160a060020a03918216600090815260056020908152604080832093909416825291909152205490565b61105f823383610870565b600654600160a060020a0316331461125857600080fd5b610cd281611a3e565b600a60209081526000928352604080842090915290825290205481565b6112898383836118e3565b60001981141561105f57600160a060020a038084166000908152600a60209081526040808320938616835292905290812055505050565b6000903b1190565b6000828211156112d457fe5b50900390565b818101828110156112e757fe5b92915050565b6112f682610e40565b1561105f5760408051600081526020810190915261131990849084908490611330565b151561105f57600080fd5b600061102b8383611abc565b600083600160a060020a031663a4c0ed3660e060020a028685856040516024018084600160a060020a0316600160a060020a0316815260200183815260200180602001828103825283818151815260200191508051906020019080838360005b838110156113a8578181015183820152602001611390565b50505050905090810190601f1680156113d55780820380516001836020036101000a031916815260200191505b5060408051601f198184030181529181526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fffffffff00000000000000000000000000000000000000000000000000000000909916989098178852518151919790965086955093509150819050838360005b8381101561146357818101518382015260200161144b565b50505050905090810190601f1680156114905780820380516001836020036101000a031916815260200191505b509150506000604051808303816000865af1979650505050505050565b600160a060020a0382166000908152600360205260409020548111156114d257600080fd5b600160a060020a0382166000908152600360205260409020546114fb908263ffffffff6112c816565b600160a060020a038316600090815260036020526040902055600454611527908263ffffffff6112c816565b600455604080518281529051600160a060020a038416917fcc16f5dbb4873280815c1ee09dbd06736cffcc184412cf7a71a0fdb75d397ca5919081900360200190a2604080518281529051600091600160a060020a03851691600080516020611d728339815191529181900360200190a35050565b80600160a060020a03811615156115b257600080fd5b600160a060020a03831615156115d0576115cb82611b8b565b61105f565b61105f8383611b97565b6000600854826040518082805190602001908083835b6020831061160f5780518252601f1990920191602091820191016115f0565b51815160209384036101000a6000190180199092169116179052604080519290940182900382207f190100000000000000000000000000000000000000000000000000000000000083830152602283019790975260428083019790975283518083039097018752606290910192839052855192945084935085019190508083835b602083106116af5780518252601f199092019160209182019101611690565b5181516020939093036101000a6000190180199091169216919091179052604051920182900390912095945050505050565b6000808460ff16601b14806116f957508460ff16601c145b1515611775576040805160e560020a62461bcd02815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202776272076616c60448201527f7565000000000000000000000000000000000000000000000000000000000000606482015290519081900360840190fd5b7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0831115611813576040805160e560020a62461bcd02815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c60448201527f7565000000000000000000000000000000000000000000000000000000000000606482015290519081900360840190fd5b60408051600080825260208083018085528a905260ff8916838501526060830188905260808301879052925160019360a0808501949193601f19840193928390039091019190865af115801561186d573d6000803e3d6000fd5b5050604051601f190151915050600160a060020a03811615156118da576040805160e560020a62461bcd02815260206004820152601860248201527f45434453413a20696e76616c6964207369676e61747572650000000000000000604482015290519081900360640190fd5b95945050505050565b600160a060020a0383161515611968576040805160e560020a62461bcd028152602060048201526024808201527f45524332303a20617070726f76652066726f6d20746865207a65726f2061646460448201527f7265737300000000000000000000000000000000000000000000000000000000606482015290519081900360840190fd5b600160a060020a03821615156119ee576040805160e560020a62461bcd02815260206004820152602260248201527f45524332303a20617070726f766520746f20746865207a65726f20616464726560448201527f7373000000000000000000000000000000000000000000000000000000000000606482015290519081900360840190fd5b600160a060020a0380841660008181526005602090815260408083209487168084529482529182902085905581518581529151600080516020611d928339815191529281900390910190a3505050565b600160a060020a0381161515611a5357600080fd5b600654604051600160a060020a038084169216907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a36006805473ffffffffffffffffffffffffffffffffffffffff1916600160a060020a0392909216919091179055565b33600090815260036020526040812054821115611ad857600080fd5b600160a060020a0383161515611aed57600080fd5b33600090815260036020526040902054611b0d908363ffffffff6112c816565b3360009081526003602052604080822092909255600160a060020a03851681522054611b3f908363ffffffff6112da16565b600160a060020a038416600081815260036020908152604091829020939093558051858152905191923392600080516020611d728339815191529281900390910190a350600192915050565b3031610e0a8282611c44565b604080517f70a0823100000000000000000000000000000000000000000000000000000000815230600482015290518391600091600160a060020a038416916370a0823191602480830192602092919082900301818787803b158015611bfc57600080fd5b505af1158015611c10573d6000803e3d6000fd5b505050506040513d6020811015611c2657600080fd5b5051905061106f600160a060020a038516848363ffffffff611cac16565b604051600160a060020a0383169082156108fc029083906000818181858888f193505050501515610e0a578082611c79611d41565b600160a060020a039091168152604051908190036020019082f080158015611ca5573d6000803e3d6000fd5b5050505050565b82600160a060020a031663a9059cbb83836040518363ffffffff1660e060020a0281526004018083600160a060020a0316600160a060020a0316815260200182815260200192505050600060405180830381600087803b158015611d0f57600080fd5b505af1158015611d23573d6000803e3d6000fd5b505050503d1561105f5760206000803e600051151561105f57600080fd5b604051602180611d51833901905600608060405260405160208060218339810160405251600160a060020a038116ff00ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925a165627a7a72305820b96bb0733a3e45fdddafa592f51114d0cf16cad047ad60b9b91ae91eb772c6940029"
    );

    // function setUp() public virtual {
    //   vm.etch(PERMITTABLE_TOKEN_ADDRESS, PERMITTABLE_TOKEN_DEPLOYED_CODE);
    // }

    function etchPermittableTokenAt(address addr) internal {
        vm.etch(addr, PERMITTABLE_TOKEN_DEPLOYED_CODE);
        // make deployer the owner
        vm.store(addr, bytes32(stdstore.target(addr).sig("owner()").find()), bytes32(abi.encode(msg.sender)));
        // setBridgeContract() to production bridge 0xf6A78083ca3e2a662D6dd1703c939c8aCE2e268d
        vm.store(
            addr,
            bytes32(stdstore.target(addr).sig("bridgeContract()").find()),
            bytes32(abi.encode(0xf6A78083ca3e2a662D6dd1703c939c8aCE2e268d))
        );
    }
}
