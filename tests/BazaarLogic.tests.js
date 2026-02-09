// 🧪 TEST: THE 180-DAY WILL EXECUTION
it("Should trigger the Legacy Split after 180 days of inactivity", async () => {
    // 1. Pioneer checks in (Day 0)
    await bazaar.checkIn({ from: pioneer_1 });

    // 2. We 'warp' the blockchain time forward by 181 days
    await time.increase(time.duration.days(181));

    // 3. We try to execute the Will
    await bazaar.executeWill(pioneer_1.address);

    // 4. THE PROOF: Check if Heir got 70% and Social Fund got 30%
    const heirBalance = await piToken.balanceOf(heir_1);
    const socialFundBalance = await piToken.balanceOf(socialFund);
    
    assert.equal(heirBalance, 14, "Heir should have 14 Pi (70% of 20)");
    assert.equal(socialFundBalance, 6, "Social Fund should have 6 Pi (30% of 20)");
});

// 🧪 TEST: THE 3-SIGNATURE MEDICAL RELEASE
it("Should only release 15% if 3 circle members sign", async () => {
    // 1. Two friends sign (Not enough)
    await bazaar.vouchMedical(pioneer_1, { from: friend_A });
    await bazaar.vouchMedical(pioneer_1, { from: friend_B });
    
    // 2. Try to withdraw (Should FAIL)
    await expectRevert(bazaar.withdrawMedical(pioneer_1));

    // 3. The third friend signs (The Magic Number)
    await bazaar.vouchMedical(pioneer_1, { from: friend_C });

    // 4. THE PROOF: 15% should now be released
    const pioneerBalance = await piToken.balanceOf(pioneer_1);
    assert.equal(pioneerBalance, 3, "Pioneer should receive 3 Pi (15% of 20)");
});